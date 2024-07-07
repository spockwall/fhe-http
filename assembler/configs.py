tokens = (
    "ADD",
    "SUB",
    "MUL",
    "DIV",
    "REM",
    "SHR",
    "SHL",
    "AND",
    "OR",
    "XOR",
    "NOT",
    # "NEG",
    "LPAREN",
    "RPAREN",
    "EQUALS",
    "NUMBER",
    "ID",
    "RETURN",
    "DEF",
    "COLON",
    "COMMA",
)

precedence = (
    ("left", "OR"),
    ("left", "XOR"),
    ("left", "AND"),
    ("left", "SHR", "SHL"),
    ("left", "ADD", "SUB"),
    ("left", "MUL", "DIV", "REM"),
    ("right", "NOT"),
)

symbol_op_table = {
    "ADD": "+",
    "SUB": "-",
    "MUL": "*",
    "DIV": "/",
    "REM": "%",
    "SHR": ">>",
    "SHL": "<<",
    "AND": "&",
    "OR": "|",
    "XOR": "^",
    "NOT": "~",
    # "NEG": "!",
}

symbol_op_func_table = {
    "ADD": lambda a, b: a + b,
    "SUB": lambda a, b: a - b,
    "MUL": lambda a, b: a * b,
    "DIV": lambda a, b: a // b,
    "REM": lambda a, b: a % b,
    "SHR": lambda a, b: a >> b,
    "SHL": lambda a, b: a << b,
    "AND": lambda a, b: a & b,
    "OR": lambda a, b: a | b,
    "XOR": lambda a, b: a ^ b,
    "NOT": lambda a: ~a,
    # "NEG": lambda a: -a,
}
