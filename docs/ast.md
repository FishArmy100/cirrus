# Cirrus Grammar

### Utilities
```fs
typeName        -> IDENTIFIER genericParams?
arguments       -> expression ("," expression)* ","?;
genericParams   -> "[" IDENTIFIER ( "," IDENTIFIER )* "," "]";
genericArgs     -> "[" typeName ("," typeName)* ","? "]";
parameters      -> IDENTIFIER ":" typeName ("=" expression)? (IDENTIFIER ":" typeName ("=" expression)?)* ","?
```

### Expressions
```fs
lambda -> // ...
primary -> NUMBER
        | STRING
        | IDENTIFIER
        | "(" expression ")"
        | "self"
        | "true"
        | "false"
        | blockExpr
        | typeName "." IDENTIFIER;

call        -> primary ( genericArgs "(" arguments? ")" | "." IDENTIFIER )*;
unary       -> ("!" | "-") unary | call;
factor      -> unary ( ( "/" | "*" ) unary )* ;
term        -> factor ( ( "/" | "*" ) factor )* ;
comparison  -> term ( ( ">" | "<" | ">=" | "<=" ) term )* ;
equality    -> comparison ( ( "==" | "!=" ) comparison )* ;
logicalAnd  -> equality ( "||" equality)* ;
logicalOr   -> logicalAnd ( "&&" logicalAnd)* ;

blockExpr   -> "{" statement* expression "}"

ifExpr      -> "if" expression blockExpr ("else" "if" expression blockExpr )* ("else" blockExpr)?;
matchExpr   -> // ...

expression  -> logicalOr | ifExpr | matchExpr;
```

### Statements
```fs
letStmt     -> "let" IDENTIFIER (":" typeName)? "=" expression ";";
varStmt     -> "var" IDENTIFIER (":" typeName)? "=" expression ";";
constStmt   -> "const" IDENTIFIER ":" typeName "=" expression ";";
assignStmt  -> IDENTIFIER "=" expression ";";

whereClause -> "where" (IDENTIFIER ":" typeName ( "+" typeName )* )+;
fnDecl      -> "pub"? "fn" IDENTIFIER genericParams? "(" parameters? ")" ("->" typeName)? whereClause? "{" statement* expression? "}";
structDecl  -> "pub"? "struct" IDENTIFIER genericParams? "{" parameters? "}";
interfaceDecl -> "pub"? "interface" IDENTIFIER genericParams? "{" ( "fn" IDENTIFIER genericParams? "(" parameters? ")" ( "->" typeName)? whereClause? ";")* "}";

implStmt    -> "impl" genericParams typeName "{" (fnDecl | constStmt)* "}";
implInterface -> "impl" genericParams typeName "for" typeName "{" fnDecl* "}";
```