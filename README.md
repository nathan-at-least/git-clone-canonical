# `git-clone-canonical`

Clone a git repository into a standard local path derived from the URL.

## Example

```
$ git clone-canonical 'https://github.com/nathan-at-least/git-clone-canonical'
[INFO ] repository path "/home/user/src/github.com/nathan-at-least/git-clone-canonical"
[INFO ] creating parent directory "/home/user/src/github.com/nathan-at-least"
[INFO ] cloning "https://github.com/nathan-at-least/git-clone-canonical"
Cloning into 'git-clone-canonical'...
remote: Enumerating objects: 58, done.
remote: Counting objects: 100% (58/58), done.
remote: Compressing objects: 100% (34/34), done.
remote: Total 58 (delta 17), reused 54 (delta 17), pack-reused 0
Receiving objects: 100% (58/58), 13.11 KiB | 2.62 MiB/s, done.
Resolving deltas: 100% (17/17), done.
```

## Fetching

If there is already a directory at the expected path, run `git fetch` instead:

```
$ git clone-canonical 'https://github.com/nathan-at-least/git-clone-canonical'
[INFO ] repository path "/home/user/src/github.com/nathan-at-least/git-clone-canonical"
[INFO ] fetching "https://github.com/nathan-at-least/git-clone-canonical"
From https://github.com/nathan-at-least/git-clone-canonical
 * branch            HEAD       -> FETCH_HEAD
```

That's it.
