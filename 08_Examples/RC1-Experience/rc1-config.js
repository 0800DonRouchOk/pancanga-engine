// RC1 deployment configuration.
//
// Empty string keeps the local same-origin backend:
// http://127.0.0.1:7878/api/calculate
//
// For zero-cost tester deployment, set this to the hosted Render backend:
window.RC1_API_BASE = "https://pancanga-engine-rc1-api.onrender.com";

window.RC1_BUILD = {
  engine: "v1.0 RC1",
  knowledgeBase: "HBV v1.0",
  astronomy: "Swiss Certified",
  build: "2026.07 RC1",
};
