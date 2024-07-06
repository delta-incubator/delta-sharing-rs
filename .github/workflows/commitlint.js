// @ts-check
/** @param {import('github-script').AsyncFunctionArguments} AsyncFunctionArguments */
export default async ({ core, context, github }) => {
  const message = `**ACTION NEEDED**

    This project follows the [Conventional Commits\
    specification](https://www.conventionalcommits.org/en/v1.0.0/) for\
    release automation.

    The PR title and description are used as the merge commit message.\
    Please update your PR title and description to match the specification.
    `;

  // Get list of current comments
  const comments = await github.paginate(github.rest.issues.listComments, {
    owner: context.repo.owner,
    repo: context.repo.repo,
    issue_number: context.issue.number,
  });

  // Check if this job already commented
  for (const comment of comments) {
    if (comment.body === message) {
      return; // Already commented
    }
  }

  // Post the comment about Conventional Commits
  github.rest.issues.createComment({
    owner: context.repo.owner,
    repo: context.repo.repo,
    issue_number: context.issue.number,
    body: message,
  });

  core.setFailed(message);
};
