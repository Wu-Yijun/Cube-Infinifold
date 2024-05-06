module.exports = async ({github, context, core, sha}) => {
  // get the latest tag (first tag in the list)
  const {tag_name, commit: {tag_sha}} = (await github.rest.repos.listTags({
    owner: context.repo.owner,  // owner of the repo
    repo: context.repo.repo,    // name of the repo
    per_page: 1                 // only need the first tag
  }))[0];
  // extract the version number from the tag (v1.2.3.4 => major=1, minor=2, patch=3, build=4)
  // need to convert the version numbers from string to number
  const [major, minor, patch, build] = tag_name.substr(1).split('.').map(Number);
  // increment the patch number and change build to running number
  const new_tag_name = `v${major}.${minor}.${patch + 1}.${context.runNumber}`;

  
}