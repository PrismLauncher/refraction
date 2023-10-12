import { Octokit } from '@octokit/core';
import { paginateRest } from '@octokit/plugin-paginate-rest';
import { throttling } from '@octokit/plugin-throttling';
import { retry } from '@octokit/plugin-retry';

import { CronJob } from 'cron';

import { storeGitHubContributors } from './storage';
import config from './config';

const MyOctokit = Octokit.plugin(paginateRest, throttling, retry);

const octokit = new MyOctokit({
  auth: config.github.token,
  throttle: {
    onRateLimit: (retryAfter, options) => {
      octokit.log.warn(
        // @ts-expect-error plugin-throttling doesn't have proper typedef for options
        `Request quota exhausted for request ${options.method} ${options.url}`
      );

      // Retry twice after hitting a rate limit error, then give up
      // @ts-expect-error plugin-throttling doesn't have proper typedef for options
      if (options.request.retryCount <= 2) {
        octokit.log.info(`Retrying after ${retryAfter} seconds!`);
        return true;
      }
    },
    onSecondaryRateLimit: (_, options, octokit) => {
      // does not retry, only logs a warning
      octokit.log.warn(
        // @ts-expect-error plugin-throttling doesn't have proper typedef for options
        `Secondary quota detected for request ${options.method} ${options.url}`
      );
    },
  },
});

const scheduledJobs: CronJob<null, null>[] = [];

const enqueueGitHubContributorsJob = (owner: string, repo: string) => {
  return new CronJob(
    config.github.updateJobCron,
    async () => {
      const contributors = await octokit.paginate(
        'GET /repos/{owner}/{repo}/contributors',
        { owner, repo }
      );
      const contributorIds = contributors.reduce((result, contributor) => {
        if (contributor.id) result.push(contributor.id.toString());
        return result;
      }, [] as string[]);

      await storeGitHubContributors(owner, repo, contributorIds);
    },
    null,
    true, // start
    null,
    null,
    true // runOnInit
  );
};

export const scheduleJobs = () => {
  for (const repo of config.github.repos) {
    scheduledJobs.push(enqueueGitHubContributorsJob(repo.owner, repo.repo));
  }
};

export const discardAllJobs = () => {
  while (scheduledJobs.length > 0) {
    scheduledJobs.pop()!.stop();
  }
};
