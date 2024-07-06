/**
 * @type {import('@commitlint/types').UserConfig}
 */
export default {
  extends: ["@commitlint/config-conventional"],
  // Workaround for https://github.com/dependabot/dependabot-core/issues/5923
  ignores: [(message) => /^Bumps \[.+]\(.+\) from .+ to .+\.$/m.test(message)],
};
