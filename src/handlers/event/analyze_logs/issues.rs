use crate::{api, utils::semver_split, Data};

use std::{io::Cursor, sync::OnceLock};

use eyre::Result;
use log::trace;
use polyhedron::{ReadLog, issues::{format::FormattedIssueInfo, issue::Issue}, read_log};
use regex::Regex;

pub type FormattedIssue = Option<(String, String)>;

pub async fn find(log: &str, data: &Data) -> Result<Vec<(String, String)>> {
	trace!("Checking log for issues");

	let mut res: Vec<(String, String)> = Vec::new();

	match read_log(Cursor::new(log), true) {
		Ok(analyzed_log) => {
			for issue in &analyzed_log.issues {
				let issue: Option<(String, String)> = format_issue(issue, &analyzed_log);
				if let Some(issue) = issue {
					res.push(issue);
				}
			}
		},
		Err(error) => { // This only happens if the log is empty or the string is corrupted
			trace!("Polyhedron failed to analyze log because: {error}");
		},
	}

	if let Some(issue) = outdated_launcher(log, data).await? {
		res.push(issue);
	}

	Ok(res)
}

async fn outdated_launcher(log: &str, data: &Data) -> Result<FormattedIssue> {
	static OUTDATED_LAUNCHER_REGEX: OnceLock<Regex> = OnceLock::new();
	let outdated_launcher = OUTDATED_LAUNCHER_REGEX.get_or_init(|| {
		Regex::new("Prism Launcher version: ((?:([0-9]+)\\.)?([0-9]+)\\.([0-9]+))").unwrap()
	});

	let Some(captures) = outdated_launcher.captures(log) else {
		return Ok(None);
	};

	let octocrab = &data.octocrab;
	let log_version = &captures[1];
	let log_version_parts = semver_split(log_version);

	let latest_version = if let Some(storage) = &data.storage {
		if let Ok(version) = storage.launcher_version().await {
			version
		} else {
			let version = api::github::get_latest_prism_version(octocrab).await?;
			storage.cache_launcher_version(&version).await?;
			version
		}
	} else {
		trace!("Not caching launcher version, as we're running without a storage backend");
		api::github::get_latest_prism_version(octocrab).await?
	};
	let latest_version_parts = semver_split(&latest_version);

	if log_version_parts.len() != 3
		|| log_version_parts[0] < latest_version_parts[0]
		|| (log_version_parts[0] == latest_version_parts[0]
			&& log_version_parts[1] < latest_version_parts[1])
		|| (log_version_parts[0] == latest_version_parts[0]
			&& log_version_parts[1] == latest_version_parts[1]
			&& log_version_parts[2] < latest_version_parts[2])
	{
		let issue = if log_version_parts[0] < 8 {
			(
				"Outdated Prism Launcher".to_string(),
				format!("Your installed version is {log_version}, while the newest version is {latest_version}.\nPlease update; for more info see https://prismlauncher.org/download/")
			)
		} else {
			(
				"Outdated Prism Launcher".to_string(),
				format!("Your installed version is {log_version}, while the newest version is {latest_version}.\nPlease update by pressing the `Update` button in the launcher or using your package manager.")
			)
		};

		Ok(Some(issue))
	} else {
		Ok(None)
	}
}

#[allow(clippy::too_many_lines)]
fn format_issue(issue: &Issue, analyzed_log: &ReadLog) -> Option<(String, String)> {
	match issue {
		Issue::OutdatedFlatpakNvidiaDriver => {
			Some((
				"Outdated Nvidia Flatpak Driver".to_string(),
				"The Nvidia driver for flatpak is outdated.
				Please run `flatpak update` to fix this issue. \
				If that does not solve it, \
				please wait until the driver is added to Flathub and run it again.".to_string(),
			))
		},
		Issue::FabricInternalAccess => {
			Some((
				"Fabric Internal Access".to_string(),
				"The mod you are using is using fabric internals that are not meant \
				to be used by anything but the loader itself.
				Those mods break both on Quilt and with fabric updates.
				If you're using fabric, downgrade your fabric loader could work, \
				on Quilt you can try updating to the latest beta version, \
				but there's nothing much to do unless the mod author stops using them.".to_string(),
			))
		},
		Issue::LexforgeZlibng => {
			Some((
				"Forge on zlib-ng".to_string(),
				"Some Linux distributions like CachyOS and Fedora
				use zlib-ng instead of classic zlib, which is 
				incompatible with MinecraftForge installers. \
				To fix this, add `-Dforgewrapper.skipHashCheck=true`
				in Edit > Settings > Java > Java arguments.".to_string(),
			))
		},
		Issue::ForgeJava => {
			Some((
				"Forge Java Bug".to_string(),
				"Old versions of Forge crash with Java 8u321+.
				To fix this, update forge to the latest version via the Versions tab
				(right click on Forge, click Change Version, and choose the latest one)
				Alternatively, you can download 8u312 or lower. \
				See [archive](https://github.com/adoptium/temurin8-binaries/releases/tag/jdk8u312-b07)".to_string(),
			))
		},
		Issue::IntelHd => {
			Some((
				"Intel HD Windows 10".to_string(),
				"Your drivers don't support Windows 10 officially
				See https://prismlauncher.org/wiki/getting-started/installing-java/#a-note-about-intel-hd-20003000-on-windows-10 for more info".to_string()
			))
		},
		Issue::JavaOption(arg) => {
			let title = if arg == "-XX:UseShenandoahGC" {
				"Java 8 and below don't support ShenandoahGC"
			} else {
				"Wrong Java Arguments"
			};
			Some((
				title.to_string(),
				format!("Remove `{arg}` from your Java arguments"),
			))
		},
		Issue::Lwjgl2JavaAbove8 => {
			Some((
				"Linux: crash with pre-1.13 and Java 9+".to_string(),
				"Using pre-1.13 (which uses LWJGL 2) with Java 9 or later usually causes a crash. \
				Switching to Java 8 or below will fix your issue.
				Alternatively, you can use [Temurin](https://adoptium.net/temurin/releases). \
				However, multiplayer will not work in versions from 1.8 to 1.11.
				For more information, type `/tag java`.".to_string(),
			))
		},
		Issue::MacOSNSInternal => {
			Some((
				"MacOS NSInternalInconsistencyException".to_string(),
				"You need to downgrade your Java 8 version. See https://prismlauncher.org/wiki/getting-started/installing-java/#older-minecraft-on-macos".to_string()
			))
		},
		Issue::Optifine => {
			Some((
				"Potential OptiFine Incompatibilities".to_string(),
				"OptiFine is known to cause problems when paired with other mods. \
				Try to disable OptiFine and see if the issue persists.
				Check `/tag optifine` for more info & some typically more compatible alternatives you can use.".to_string(),
			))
		},
		Issue::OutdatedLauncher => None, // Offline version, networked check is better
		Issue::NettyJavaAbove8 => {
			Some((
				"Linux: broken multiplayer with 1.8-1.11 and Java 9+".to_string(),
				"These versions of Minecraft use an outdated version of Netty which does not properly support Java 9.

				Switching to Java 8 or below will fix this issue. For more information, type `/tag java`.

				If you must use a newer version, do the following:
				- Open `options.txt` (in the main window Edit -> Open .minecraft) and change.
				- Find `useNativeTransport:true` and change it to `useNativeTransport:false`.
				Note: whilst Netty was introduced in 1.7, this option did not exist \
				which is why the issue was not present.".to_string(),
			))
		},
		Issue::WrongJava(version) => {
			let description = match (&analyzed_log.recommended_java_version, version) {
				(Some(version), _) => format!("Please switch to Java version {}{}\nFor more information, type `/tag java`", version.major, version.minor.map_or_else(String::new, |v| format!(" {v}"))),
				(_, Some(version)) => format!("Please switch to Java version {version}\nFor more information, type `/tag java`"),
				(_, _) => "Please switch to another Java version\nFor more information, type `/tag java`".to_string(),
			};
			Some((
				"Wrong Java Version".to_string(),
				description,
			))
		},
		Issue::ForgeMissingDependencies => {
			Some((
				"Missing mod dependencies".to_string(),
				"You seem to be missing mod dependencies.
				Search for \"mandatory dependencies\" in your log.".to_string(),
			))
		},
		Issue::NewJavaOldForgeLegacyJavaFixer => {
			Some((
				"LegacyJavaFixer".to_string(),
				"You are using a modern Java version with an old Forge version, which is causing this crash.
				MinecraftForge provides a coremod to fix this issue, download it [here](https://dist.creeper.host/FTB2/maven/net/minecraftforge/lex/legacyjavafixer/1.0/legacyjavafixer-1.0.jar).".to_string(),
			))
		},
		Issue::LockedJars(jars) => {
			Some((
				format!("{} Locked Jars", jars.len()),
				"Something is locking your library jars.".to_string(),
			))
		},
		Issue::MissingLibraries(libs) => {
			Some((
				format!("{} Missing Libraries", libs.len()),
				"You seem to be missing libraries. This is usually caused by launching offline before they can be downloaded.
				To fix this, first ensure you are connected to the internet. Then, try selecting Edit > Version > Download All and launching your instance again.
				If Minecraft is getting launched offline by default, it's possible your token got expired. To fix this, remove and add back your Microsoft account.".to_string(),
			))
		},
		Issue::MissingIndium => {
			Some((
				"Missing Indium".to_string(),
				"You are using a mod that needs Indium.
				Please install it by going to Edit > Mods > Download Mods.".to_string(),
			))
		},
		Issue::Java32BitMemoryLimit => {
			Some((
				"32 bit Java crash".to_string(),
				"You are using a 32 bit Java version. Please select 64 bit Java instead. Check `/tag java` for more information.".to_string(),
			))
		},
		Issue::WrongIntermediaryMappingsVersion => {
			Some((
				"Wrong Intermediary Mappings version".to_string(),
				"You are using Intermediary Mappings for the wrong Minecraft version.
				Please select Change Version while it is selected in Edit > Version.".to_string(),
			))
		},
		Issue::NewJavaOldForgeIgnoreCerts => {
			Some((
				"Forge on old Minecraft versions".to_string(),
				"This crash is caused by using an old Forge version with a modern Java version.
				To fix it, add the flag `-Dfml.ignoreInvalidMinecraftCertificates=true` to Edit > Settings > Java arguments.".to_string(),
			))
		},
		Issue::ChecksumMismatch => {
			Some((
				"Outdated cached files".to_string(),
				"It looks like you need to delete cached files.
				To do that, press Folders ⟶ View Launcher Root Folder, and **after closing the launcher** delete the folder named \"meta\".".to_string(),
			))
		},
		Issue::NvidiaLinux => {
			Some((
				"Nvidia drivers on Linux".to_string(),
				"Nvidia drivers will often cause crashes on Linux.
				To fix it, go to Settings ⟶ Enviroment variables and set `__GL_THREADED_OPTIMIZATIONS` to `0`.".to_string(),
			))
		},
		Issue::LinuxOpenal => {
			Some((
				"Missing .alsoftrc".to_string(),
				"OpenAL is likely missing the configuration file.
				To fix this, create a file named `.alsoftrc` in your home directory by running this command in your terminal:
				```
echo -e \"drivers=alsa\\nhrtf=true\" > ~/.alsoftrc```"
					.to_string(),
			))
		},
		Issue::X11ConnectFailure => {
			Some((
				"Flatpak crash".to_string(),
				"To fix this crash, disable \"Fallback to X11 Windowing System\" in Flatseal.".to_string(),
			))
		},
		Issue::OldJavaMacOs => {
			Some((
				"Old Java on MacOS".to_string(),
				"This crash is caused by an old Java version conflicting with mods, most often Spark, on MacOS.
				To fix it, either remove Spark or update Java by going to Edit > Settings > Download Java > Adoptium, and selecting the new Java version via Auto-Detect.".to_string(),
			))
		},
		Issue::MissingXrandr => {
			Some((
				"Missing xrandr".to_string(),
				"This crash is caused by not having xrandr installed on Linux on Minecraft versions that use LWJGL 2.".to_string(),
			))
		},
		Issue::InstanceDataCorrupted => {
			Some((
				"Corrupted instance files".to_string(),
				"Your instance's `mmc-pack.json` appears to be corrupted. Make a new instance and copy over your data between `.minecraft` folders. To prevent this in the future, ensure your system has sufficient disk space and avoid forcefully shutting down your PC.".to_string(),
			))
		},
		Issue::InvalidProxy => {
			Some((
				"Invalid proxy configuration".to_string(),
				"Your proxy configuration in Prism settings seems to be incorrect.
				Try undoing it by selecing \"None\" in top toolbar > **Settings** > **Proxy**.".to_string(),
			))
		},
		Issue::ShaderCompileError | Issue::MixinApplyFailure(_) | Issue::InstanceUpdateFailed(_) | Issue::ErrorInitializationVM | Issue::ZipExtractFailure | 
		Issue::ForgeSuspectedMod(_) | Issue::EntrypointExecutionErrors(_) | Issue::CriticalInjectionFailure(_) | Issue::ModsFoundInStacktraceNamespace(_) | 
		Issue::ModsFoundInStacktraceInfo(_) | Issue::Oom | Issue::InvalidFolderName(_) | Issue::NoDiskSpace | Issue::FatalErrorJre(_) | Issue::IncompatibleMods(_) => {
			let formatted = FormattedIssueInfo::from_issue(issue);
			Some((
				formatted.title,
				formatted.description
			))
		},
	}
}