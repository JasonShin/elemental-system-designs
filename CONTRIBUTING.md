# Contributing to elemental-system-designs

All sort of contributions are welcome and there are no complicated rules with it.
We appreciate:

- New System Architecture Design
- Bug fixes
- Suggestions
- Ideas

## Issues

Feel free to submit issues, ideas, suggestions and enhancement requests.

## Contributing

Please refer to each project's style guidelines and guidelines for submitting patches and additions.
In general, we follow the "fork-and-pull" Git workflow.

1.  **Fork** the repo on GitHub
2.  **Clone** the project to your own machine
3.  **Commit** changes to your own branch
4.  **Push** your work back up to your fork
5.  Submit a **Pull request** so that we can review your changes

NOTE: Be sure to merge the latest from "upstream" before making a pull request!

## Development Environments

| type  | version                |
|-------|------------------------|
| Rustc | latest nightly         |
| OS    | Linux, Windows and Mac |

## Releasing

Elemental System Designs project focused on high level design of a system and do not directly implement a system from ground up. Releasing will introduce a new design of a system, bug fixes to the existing system designs or improvements.

#### Making a release

1. Confirm the current status of commits in the master branch and create a tag based on the commits in master
```bash
git tag -a v1.2.3
git push v1.2.3
```

We follow semver for our releases

2. Update the changelog on the Github release

## Copyright and Licensing

elemental-system-designs is an open source project licensed under the MIT license.

elemental-system-designs does not require you to assign the copyright of your contributions, you retain the copyright.
elemental-system-designs does require that you make your contributions available under the MIT license in order to be
included in the main repo.

If appropriate, include the MIT license summary at the top of each file along with the copyright info.
If you are adding a new file that you wrote, include your name in the copyright notice in the license
summary at the top of the file.

## License Summary

You can copy and paste the MIT license summary from below.

```
MIT License

Copyright (c) 2022 Jason Shin

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```
