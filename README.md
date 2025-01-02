# jsonedit

A basic CLI for precise editing of JSON files using JSONPath queries.

```
Usage: jsonedit <FILE> <QUERY> <COMMAND>

Commands:
  set   Set a property on exactly one object returned by the query
  help  Print this message or the help of the given subcommand(s)

Arguments:
  <FILE>   Path to JSON file
  <QUERY>  A valid JSONPath query according to RFC 9535

Options:
  -h, --help  Print help
```
