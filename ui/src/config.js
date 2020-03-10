// console.log(process);
export const INSTANCE_NAME = process.env.PRODUCTION
  ? 'learning-pathways'
  : 'test-instance';
export const COURSES_ZOME = 'courses';
export const MEMBERS_ZOME = 'members';
export const HOST_URL = process.env.WS_INTERFACE;
export const USERNAME = process.env.USERNAME;
