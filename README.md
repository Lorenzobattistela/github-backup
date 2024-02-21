# Github Backup tool

Are all your repos centralized in github? Maybe it's time for a backup.
Keep local copies of your repos to avoid having it all wiped out for some reason!

This is a tool / learning project to use Rust.

## Quickstart

### Setting up env

For this project to work, you'll need a github api token. You can check how to do this [here](https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api?apiVersion=2022-11-28#3-create-an-access-token).
After creating the token and granting reading access of repos, place it at `.env` file.

### CLI arguments

The arguments are sequential, and they are:
- output_path : Where the zipped repos are going to be placed. Defaults to `./backups`
- --allow-others_repos : This allows or not to backup repos that were not created by you, but are in your "repositories" page. Defaults to `false`

Example valid running commands are:

`./github-backup ./backups --allow-others_repos`
`./github-backup ./backups`
`./github-backup`

Note that you cant pass the allow-others-repos without passing the output_path.

### Running the Binary

If you're not a rust person and do not pretend to change any of this code, simply run the executable placed on root.

`./github-backup`


### Running with Cargo

If you are a rust person and pretend to change the code, you can run it with cargo!

`cargo run`

and pass the arguments if wanted.
