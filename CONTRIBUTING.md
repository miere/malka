# Contributing

Thank you for your interest in contributing to Malka! There are many ways to contribute and we
appreciate all of them.

1. Found a bug or want to request a feature? Please open a ticket for it.
2. Found a typo or a topic that needs to be improved in the documentation? Just open a PR for us.
3. Has something in mind that would enhance Malka? Put some lines of code together and open a PR.
   We'd love to discuss your ideas. Even half-baked ideas will be welcomed, just make sure to make
   a good point in the Pull Request description.

Please note we have a code of conduct, please follow it in all your interactions with the project.

## Reporting Bugs/Feature Requests
We welcome you to use the GitHub issue tracker to report bugs or suggest features.

When filing an issue, please check existing open, or recently closed, issues to make sure somebody else hasn't already
reported the issue. Please try to include as much information as you can. Details like these are incredibly useful:

* A reproducible test case or series of steps
* The version of our code being used
* Any modifications you've made relevant to the bug
* Anything unusual about your environment or deployment


## Contributing via Pull Requests
Contributions via pull requests are much appreciated. Before sending us a pull request, please ensure that:

1. You are working against the latest source on the *main* branch.
2. You check existing open, and recently merged, pull requests to make sure someone else hasn't addressed the problem already.
3. You open an issue to discuss any significant work - we would hate for your time to be wasted.

To send us a pull request, please:

1. Fork the repository.
2. Modify the source; please focus on the specific change you are contributing. If you also reformat all the code, it will be hard for us to focus on your change.
3. Ensure local tests pass.
4. Commit to your fork using clear commit messages.
5. Send us a pull request, answering any default questions in the pull request interface.
6. Pay attention to any automated CI failures reported in the pull request, and stay involved in the conversation.

GitHub provides additional document on [forking a repository](https://help.github.com/articles/fork-a-repo/) and
[creating a pull request](https://help.github.com/articles/creating-a-pull-request/).


## Finding contributions to work on
Looking at the existing issues is a great way to find something to contribute on. As our projects, by default, use the default GitHub issue labels ((enhancement/bug/duplicate/help wanted/invalid/question/wontfix), looking at any 'help wanted' issues is a great place to start.

## Release
Not all modifications to the `main` branch becomes a new release. Once merged to master a throught code
check will be performed (from regression tests to linting). If everything goes green, one of
the maintainers will create a tag from the desired point in the `main` branch. Once the tag is pushed
to the repository, a new release will be automatically issued to DockerHub.

Please, ensure that you have increased the version numbers in the project, as well as in any examples files
and the README.md, to the new version that this new release would represent. The versioning scheme we
use is [SemVer](http://semver.org/).
