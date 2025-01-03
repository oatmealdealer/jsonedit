# jsonedit

A basic CLI for precise editing of JSON files using JSONPath queries.

```
Usage: jsonedit [FILES]... <QUERY> <COMMAND>

Commands:
  set   Set a property on any objects returned by the query
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [FILES]...  Paths to one or more JSON files
  <QUERY>     A valid JSONPath query according to RFC 9535

Options:
  -h, --help  Print help
```
