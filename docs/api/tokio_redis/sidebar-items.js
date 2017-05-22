initSidebarItems({"enum":[["ErrorKind","An enum of all error kinds."],["Value","Internal low-level redis value enum."]],"struct":[["Client",""],["Cmd","Represents redis commands."],["Error","Represents a redis error.  For the most part you should be using the Error trait to interact with this rather than the actual struct."],["Okay",""],["Redis",""]],"trait":[["FromRedisValue","This trait is used to convert a redis value into a more appropriate type.  While a redis `Value` can represent any response that comes back from the redis server, usually you want to map this into something that works better in rust.  For instance you might want to convert the return value into a `String` or an integer."],["ToRedisArgs","Used to convert a value into one or multiple redis argument strings.  Most values will produce exactly one item but in some cases it might make sense to produce more than one."]],"type":[["Response",""],["Result","Library generic result type."]]});