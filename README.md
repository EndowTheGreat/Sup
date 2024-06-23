<h1 align=center>Sup</h1>

**Sup** is an easy-to-use Linux CLI aimed at simplifying local:remote file syncing.

----

<h2>Installation</h2>

```
$ git clone https://gitlab.com/EndowTheGreat/sup.git
$ cd sup
$ make
```

----

<h2>Configuration</h2>
<p>Whenever Sup is initially ran, a default configuration file will get generated in <code>.sup/config.toml</code> that will look like this: </p>

```toml
username = "root"
key_file = "/home/user/.ssh/id_rsa.pub"
server = "ssh.example.com"
```
<p>Configure this so it works for you, and then you're all ready to move on!</p>

<h2>Usage</h2>

```
Usage: sup [OPTIONS] --remote-dir <REMOTE_DIR>

Options:
  -d, --directory <DIRECTORY>    Local directory to upload to the remote server. [default: .]
  -r, --remote-dir <REMOTE_DIR>  Remote directory path for the upload.
  -s, --skip <SKIP>              Files or directories to ignore (separated by ','). [default: ]
  -i, --ignore                   Ignore dotfiles when uploading.
  -h, --help                     Print help
```

<h2>Contributing</h2>

Pull requests and contributions are absolutely welcome, feel free to fork or improve upon my work however you wish. To make things nice and easy, you can open a PR [here](https://gitlab.com/EndowTheGreat/sup/-/merge_requests/new).