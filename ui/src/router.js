import Navigo from 'navigo';

export const router = new Navigo(
  process.env.PRODUCTION ? null : `http://localhost:${window.location.port}`
);
