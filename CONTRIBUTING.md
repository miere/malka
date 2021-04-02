# Contributing

Thank you for your interest in contributing to Malka! There are many ways to contribute and we
appreciate all of them.

1. Found a bug or want to request a feature? Please open a ticket for it.
2. Found a typo or a topic that needs to be improved in the documentation? Just open a PR for us.
3. Has something in mind that would enhance Malka? Put some lines of code together and open a PR.
   We'd love to discuss your ideas. Even half-baked ideas will be welcomed, just make sure to make
   a good point in the Pull Request description.

Please note we have a code of conduct, please follow it in all your interactions with the project.

## Pull Request Process

1. Ensure any install or build dependencies are removed before the end of the layer when doing a 
   build.
2. Update the README.md with details of changes to the interface, this includes new environment 
   variables, exposed ports, useful file locations and container parameters.
4. You may merge the Pull Request in once you have the sign-off of any maintainer, or if you 
   do not have permission to do that, you may request the second reviewer to merge it for you.

## Release
Not all modifications to the `main` branch becomes a new release. Once merged to master a throught code
check will be performed (from regression tests to linting). If everything goes green, one of
the maintainers will create a tag from the desired point in the `main` branch. Once the tag is pushed
to the repository, a new release will be automatically issued to DockerHub.

Please, ensure that you have increased the version numbers in the project, as well as in any examples files
and the README.md, to the new version that this new release would represent. The versioning scheme we
use is [SemVer](http://semver.org/).
