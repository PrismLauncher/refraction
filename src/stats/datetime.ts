import { addMinutes, format } from 'date-fns';

export const getTimeIntervalID = () => {
  const today = new Date();

  return format(addMinutes(today, today.getTimezoneOffset()), 'yyyy.MM.dd.HH');
};
