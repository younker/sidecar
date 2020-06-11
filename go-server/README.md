# Go Server

Example:

```bash
# term 1
$ go-server 1234

# term 2
$ nc 127.0.0.1 1234
echo this

# term 1
$ go-server 1234
.echo this

# term 2
$ nc 127.0.0.1 1234
echo this
STOP
$
```

## Go Setup

`/Users/jasonyounker/go/bin` is exported in `PATH` making the binaries available:

```bash
$ which go-server
/Users/jasonyounker/go/bin/go-server
```

From the [Your first program](https://golang.org/doc/code.html#Command) section:

```
The install directory is controlled by the GOPATH and GOBIN environment variables. If GOBIN is set, binaries are installed to that directory. If GOPATH is set, binaries are installed to the bin subdirectory of the first directory in the GOPATH list. Otherwise, binaries are installed to the bin subdirectory of the default GOPATH ($HOME/go or %USERPROFILE%\go).
```

## Resources
- https://golang.org/doc/code.html#Command
- https://blog.golang.org/using-go-modules
- https://github.com/golang/go/wiki/Modules
- https://github.com/golang/vscode-go/blob/master/docs/modules.md
- https://www.linode.com/docs/development/go/developing-udp-and-tcp-clients-and-servers-in-go/#create-a-concurrent-tcp-server
