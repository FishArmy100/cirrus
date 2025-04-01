# Cirrus Grammar

### Utilities
```fs
typeName  	-> (IDENTIFIER | Self) genericArgs? ( "." IDENTIFIER genericArgs? )*;
            | "[" "]" typeName 
			| "fn" "(" ( typeName ("," typeName)* )? ")" ( "->" typeName)?;


arguments       -> expression ("," expression)* ","?;
genericParams   -> "[" IDENTIFIER ( "," IDENTIFIER )* "," "]";
genericArgs     -> "[" typeName ("," typeName)* ","? "]";
parameters      -> "var"? IDENTIFIER ":" typeName ("=" expression)? (IDENTIFIER ":" typeName ("=" expression)?)* ","?;

patternFields	-> IDENTIFIER (":" pattern) ("," IDENTIFIER (":" pattern))*;
pattern			-> NUMBER | STRING | IDENTIFIER ("." IDENTIFIER)* ("(" pattern ")")? | typeName ( "{" patternField "}" | );

```

### Expressions
```fs
lambda -> "|" (IDENTIFIER (":" typeName)? ("," IDENTIFIER (":" typeName)?)* ","? "|" ("->" typeName)?) "=>" expression
primary -> NUMBER
        | STRING
        | IDENTIFIER
        | "(" expression ")"
        | "self"
        | "true"
        | "false"
        | blockExpr
        | lambda
        | typeName "." IDENTIFIER
		| typeName "{" (IDENTIFIER ":" expression ("," IDENTIFIER ":" expression)? "," )? "}";
        | blockExpr

call        -> primary ( genericArgs "(" arguments? ")" | "[" expression "]" | "." IDENTIFIER )*;
unary       -> ("!" | "-") unary | call;
factor      -> unary ( ( "/" | "*" ) unary )* ;
term        -> factor ( ( "+" | "-" ) factor )* ;
comparison  -> term ( ( ">" | "<" | ">=" | "<=" ) term )* ;
equality    -> comparison ( ( "==" | "!=" ) comparison )* ;
logicalAnd  -> equality ( "||" equality)* ;
logicalOr   -> logicalAnd ( "&&" logicalAnd)* ;

blockExpr   -> "{" statement* expression? "}";

ifCond		-> expression | "let" pattern "=" expression ("&&" ifCond )?;
ifExpr      -> "if" ifCond blockExpr ("else" (ifExpr | blockExpr))?;
matchExpr   -> "match" expression "{" pattern "=>" expression ("," pattern "=>" expression)* ","? "}";

expression  -> logicalOr | ifExpr | matchExpr;
```

### Statements
```fs
useStmt 	-> "use" IDENTIFIER ("." IDENTIFIER)* ("." "*")? ";";
exprStmt	-> expression ";";
letStmt     -> "let" IDENTIFIER (":" typeName)? "=" expression ";";
varStmt     -> "var" IDENTIFIER (":" typeName)? "=" expression ";";
constStmt   -> "const" IDENTIFIER ":" typeName "=" expression ";";
assignStmt  -> IDENTIFIER "=" expression ";";

whereClause -> "where" (IDENTIFIER ":" typeName ( "+" typeName )* )+;
fnDecl      -> "fn" IDENTIFIER genericParams? "(" parameters? ")" ("->" typeName)? whereClause? "{" statement* expression? "}";

structParam	-> "pub"? "var"? IDENTIFIER ":" typeName ("=" expression)?
structDecl  -> "struct" IDENTIFIER genericParams? "{" ( structParam ("," structParam)* )? "}";
interfaceDecl -> "interface" IDENTIFIER genericParams? "{" ( "fn" IDENTIFIER genericParams? "(" parameters? ")" ( "->" typeName)? whereClause? ";")* "}";
enumDecl	-> "enum" IDENTIFIER genericParams? whereClause? "{" IDENTIFIER ( "(" typeName ")" | "{" parameters? "}") "}";
typeDecl	-> "type" IDENTIFIER genericParams? "=" typeName ";";

implStmt    -> "impl" genericParams typeName ("for") typeName "{" (fnDecl | constStmt)* "}";

breakStmt   -> "break"+ ";";
breakStmt   -> "continue"+ ";";

statement	-> letStmt | varStmt | constStmt | assignStmt | ifExpr | matchExpr | blockExpr | exprStmt | useStmt;
declaration	-> "pub" (fnDecl | structDecl | interfaceDecl | enumDecl | typeDecl | letStmt | varStmt | constStmt | useStmt) | implStmt;
program -> declStmt*;
```