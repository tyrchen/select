# "select" command line tool

"select" is a command line tool to allow you to run SQL query against local file, remote url and command output. This is a very early experimental tool.

Examples:

Query with command:
![](docs/images/command.jpg)

Query with local file:
![](docs/images/file.jpg)

Query with remote file:
![](docs/images/url.jpg)

currently only csv files/urls are supported. For command output, I did a hack to retrieve columns through awk then convert it to csv, so the result might be incorrect.
