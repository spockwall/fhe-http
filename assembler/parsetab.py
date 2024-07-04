
# parsetab.py
# This file is automatically generated. Do not edit.
# pylint: disable=W,C,R
_tabversion = '3.10'

_lr_method = 'LALR'

_lr_signature = 'leftORleftXORleftANDleftSHRSHLleftADDSUBleftMULDIVREMrightNOTNEGADD AND COLON DEF DIV EQUALS ID LPAREN MUL NEG NOT NUMBER OR REM RETURN RPAREN SHL SHR SUB XORfunction : DEF ID LPAREN RPAREN COLON statements RETURN expressionstatements : statements statement\n        | statementstatement : ID EQUALS expressionexpression : expression ADD expression\n        | expression SUB expression\n        | expression MUL expression\n        | expression DIV expression\n        | expression REM expression\n        | expression SHR expression\n        | expression SHL expression\n        | expression AND expression\n        | expression OR expression\n        | expression XOR expressionexpression : NEG expression\n        | NOT expressionexpression : LPAREN expression RPARENexpression : NUMBERexpression : ID'
    
_lr_action_items = {'DEF':([0,],[2,]),'$end':([1,13,18,19,30,31,33,34,35,36,37,38,39,40,41,42,43,],[0,-19,-18,-1,-15,-16,-5,-6,-7,-8,-9,-10,-11,-12,-13,-14,-17,]),'ID':([2,6,8,9,10,11,12,13,14,15,16,17,18,20,21,22,23,24,25,26,27,28,29,30,31,33,34,35,36,37,38,39,40,41,42,43,],[3,7,7,-3,13,13,-2,-19,-4,13,13,13,-18,13,13,13,13,13,13,13,13,13,13,-15,-16,-5,-6,-7,-8,-9,-10,-11,-12,-13,-14,-17,]),'LPAREN':([3,10,11,15,16,17,20,21,22,23,24,25,26,27,28,29,],[4,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,]),'RPAREN':([4,13,18,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[5,-19,-18,-15,-16,43,-5,-6,-7,-8,-9,-10,-11,-12,-13,-14,-17,]),'COLON':([5,],[6,]),'EQUALS':([7,],[10,]),'RETURN':([8,9,12,13,14,18,30,31,33,34,35,36,37,38,39,40,41,42,43,],[11,-3,-2,-19,-4,-18,-15,-16,-5,-6,-7,-8,-9,-10,-11,-12,-13,-14,-17,]),'NEG':([10,11,15,16,17,20,21,22,23,24,25,26,27,28,29,],[15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,]),'NOT':([10,11,15,16,17,20,21,22,23,24,25,26,27,28,29,],[16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,]),'NUMBER':([10,11,15,16,17,20,21,22,23,24,25,26,27,28,29,],[18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,]),'ADD':([13,14,18,19,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[-19,20,-18,20,-15,-16,20,-5,-6,-7,-8,-9,20,20,20,20,20,-17,]),'SUB':([13,14,18,19,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[-19,21,-18,21,-15,-16,21,-5,-6,-7,-8,-9,21,21,21,21,21,-17,]),'MUL':([13,14,18,19,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[-19,22,-18,22,-15,-16,22,22,22,-7,-8,-9,22,22,22,22,22,-17,]),'DIV':([13,14,18,19,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[-19,23,-18,23,-15,-16,23,23,23,-7,-8,-9,23,23,23,23,23,-17,]),'REM':([13,14,18,19,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[-19,24,-18,24,-15,-16,24,24,24,-7,-8,-9,24,24,24,24,24,-17,]),'SHR':([13,14,18,19,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[-19,25,-18,25,-15,-16,25,-5,-6,-7,-8,-9,-10,-11,25,25,25,-17,]),'SHL':([13,14,18,19,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[-19,26,-18,26,-15,-16,26,-5,-6,-7,-8,-9,-10,-11,26,26,26,-17,]),'AND':([13,14,18,19,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[-19,27,-18,27,-15,-16,27,-5,-6,-7,-8,-9,-10,-11,-12,27,27,-17,]),'OR':([13,14,18,19,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[-19,28,-18,28,-15,-16,28,-5,-6,-7,-8,-9,-10,-11,-12,-13,-14,-17,]),'XOR':([13,14,18,19,30,31,32,33,34,35,36,37,38,39,40,41,42,43,],[-19,29,-18,29,-15,-16,29,-5,-6,-7,-8,-9,-10,-11,-12,29,-14,-17,]),}

_lr_action = {}
for _k, _v in _lr_action_items.items():
   for _x,_y in zip(_v[0],_v[1]):
      if not _x in _lr_action:  _lr_action[_x] = {}
      _lr_action[_x][_k] = _y
del _lr_action_items

_lr_goto_items = {'function':([0,],[1,]),'statements':([6,],[8,]),'statement':([6,8,],[9,12,]),'expression':([10,11,15,16,17,20,21,22,23,24,25,26,27,28,29,],[14,19,30,31,32,33,34,35,36,37,38,39,40,41,42,]),}

_lr_goto = {}
for _k, _v in _lr_goto_items.items():
   for _x, _y in zip(_v[0], _v[1]):
       if not _x in _lr_goto: _lr_goto[_x] = {}
       _lr_goto[_x][_k] = _y
del _lr_goto_items
_lr_productions = [
  ("S' -> function","S'",1,None,None,None),
  ('function -> DEF ID LPAREN RPAREN COLON statements RETURN expression','function',8,'p_function','assembler.py',73),
  ('statements -> statements statement','statements',2,'p_statements','assembler.py',79),
  ('statements -> statement','statements',1,'p_statements','assembler.py',80),
  ('statement -> ID EQUALS expression','statement',3,'p_statement','assembler.py',84),
  ('expression -> expression ADD expression','expression',3,'p_expression_binop','assembler.py',89),
  ('expression -> expression SUB expression','expression',3,'p_expression_binop','assembler.py',90),
  ('expression -> expression MUL expression','expression',3,'p_expression_binop','assembler.py',91),
  ('expression -> expression DIV expression','expression',3,'p_expression_binop','assembler.py',92),
  ('expression -> expression REM expression','expression',3,'p_expression_binop','assembler.py',93),
  ('expression -> expression SHR expression','expression',3,'p_expression_binop','assembler.py',94),
  ('expression -> expression SHL expression','expression',3,'p_expression_binop','assembler.py',95),
  ('expression -> expression AND expression','expression',3,'p_expression_binop','assembler.py',96),
  ('expression -> expression OR expression','expression',3,'p_expression_binop','assembler.py',97),
  ('expression -> expression XOR expression','expression',3,'p_expression_binop','assembler.py',98),
  ('expression -> NEG expression','expression',2,'p_expression_uniop','assembler.py',108),
  ('expression -> NOT expression','expression',2,'p_expression_uniop','assembler.py',109),
  ('expression -> LPAREN expression RPAREN','expression',3,'p_expression_group','assembler.py',117),
  ('expression -> NUMBER','expression',1,'p_expression_number','assembler.py',121),
  ('expression -> ID','expression',1,'p_expression_id','assembler.py',127),
]
