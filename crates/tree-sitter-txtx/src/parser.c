#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 153
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 68
#define ALIAS_COUNT 0
#define TOKEN_COUNT 40
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 8
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 10

enum {
  anon_sym_addon = 1,
  anon_sym_signer = 2,
  anon_sym_action = 3,
  anon_sym_output = 4,
  anon_sym_variable = 5,
  anon_sym_input = 6,
  anon_sym_EQ = 7,
  anon_sym_import = 8,
  anon_sym_LBRACE = 9,
  anon_sym_RBRACE = 10,
  anon_sym_DQUOTE = 11,
  aux_sym_string_token1 = 12,
  anon_sym_SQUOTE = 13,
  aux_sym_string_token2 = 14,
  anon_sym_DQUOTE_DQUOTE_DQUOTE = 15,
  aux_sym_string_token3 = 16,
  aux_sym_number_token1 = 17,
  aux_sym_number_token2 = 18,
  aux_sym_number_token3 = 19,
  anon_sym_true = 20,
  anon_sym_false = 21,
  sym_null = 22,
  anon_sym_LBRACK = 23,
  anon_sym_COMMA = 24,
  anon_sym_RBRACK = 25,
  anon_sym_COLON = 26,
  anon_sym_DOT = 27,
  anon_sym_LPAREN = 28,
  anon_sym_RPAREN = 29,
  anon_sym_STAR = 30,
  anon_sym_SLASH = 31,
  anon_sym_PLUS = 32,
  anon_sym_DASH = 33,
  sym_identifier = 34,
  anon_sym_POUND = 35,
  aux_sym_comment_token1 = 36,
  anon_sym_SLASH_SLASH = 37,
  anon_sym_SLASH_STAR = 38,
  aux_sym_comment_token2 = 39,
  sym_runbook = 40,
  sym__statement = 41,
  sym_addon_block = 42,
  sym_signer_block = 43,
  sym_action_block = 44,
  sym_output_block = 45,
  sym_variable_declaration = 46,
  sym_input_declaration = 47,
  sym_import_statement = 48,
  sym_block = 49,
  sym_attribute = 50,
  sym__expression = 51,
  sym_string = 52,
  sym_number = 53,
  sym_boolean = 54,
  sym_array = 55,
  sym_object = 56,
  sym_object_field = 57,
  sym_reference = 58,
  sym_index_access = 59,
  sym_function_call = 60,
  sym_binary_expression = 61,
  sym_comment = 62,
  aux_sym_runbook_repeat1 = 63,
  aux_sym_block_repeat1 = 64,
  aux_sym_array_repeat1 = 65,
  aux_sym_object_repeat1 = 66,
  aux_sym_reference_repeat1 = 67,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_addon] = "addon",
  [anon_sym_signer] = "signer",
  [anon_sym_action] = "action",
  [anon_sym_output] = "output",
  [anon_sym_variable] = "variable",
  [anon_sym_input] = "input",
  [anon_sym_EQ] = "=",
  [anon_sym_import] = "import",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_string_token1] = "string_token1",
  [anon_sym_SQUOTE] = "'",
  [aux_sym_string_token2] = "string_token2",
  [anon_sym_DQUOTE_DQUOTE_DQUOTE] = "\"\"\"",
  [aux_sym_string_token3] = "string_token3",
  [aux_sym_number_token1] = "number_token1",
  [aux_sym_number_token2] = "number_token2",
  [aux_sym_number_token3] = "number_token3",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [sym_null] = "null",
  [anon_sym_LBRACK] = "[",
  [anon_sym_COMMA] = ",",
  [anon_sym_RBRACK] = "]",
  [anon_sym_COLON] = ":",
  [anon_sym_DOT] = ".",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_STAR] = "*",
  [anon_sym_SLASH] = "/",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [sym_identifier] = "identifier",
  [anon_sym_POUND] = "#",
  [aux_sym_comment_token1] = "comment_token1",
  [anon_sym_SLASH_SLASH] = "//",
  [anon_sym_SLASH_STAR] = "/*",
  [aux_sym_comment_token2] = "comment_token2",
  [sym_runbook] = "runbook",
  [sym__statement] = "_statement",
  [sym_addon_block] = "addon_block",
  [sym_signer_block] = "signer_block",
  [sym_action_block] = "action_block",
  [sym_output_block] = "output_block",
  [sym_variable_declaration] = "variable_declaration",
  [sym_input_declaration] = "input_declaration",
  [sym_import_statement] = "import_statement",
  [sym_block] = "block",
  [sym_attribute] = "attribute",
  [sym__expression] = "_expression",
  [sym_string] = "string",
  [sym_number] = "number",
  [sym_boolean] = "boolean",
  [sym_array] = "array",
  [sym_object] = "object",
  [sym_object_field] = "object_field",
  [sym_reference] = "reference",
  [sym_index_access] = "index_access",
  [sym_function_call] = "function_call",
  [sym_binary_expression] = "binary_expression",
  [sym_comment] = "comment",
  [aux_sym_runbook_repeat1] = "runbook_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym_array_repeat1] = "array_repeat1",
  [aux_sym_object_repeat1] = "object_repeat1",
  [aux_sym_reference_repeat1] = "reference_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_addon] = anon_sym_addon,
  [anon_sym_signer] = anon_sym_signer,
  [anon_sym_action] = anon_sym_action,
  [anon_sym_output] = anon_sym_output,
  [anon_sym_variable] = anon_sym_variable,
  [anon_sym_input] = anon_sym_input,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_string_token1] = aux_sym_string_token1,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [aux_sym_string_token2] = aux_sym_string_token2,
  [anon_sym_DQUOTE_DQUOTE_DQUOTE] = anon_sym_DQUOTE_DQUOTE_DQUOTE,
  [aux_sym_string_token3] = aux_sym_string_token3,
  [aux_sym_number_token1] = aux_sym_number_token1,
  [aux_sym_number_token2] = aux_sym_number_token2,
  [aux_sym_number_token3] = aux_sym_number_token3,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [sym_null] = sym_null,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [sym_identifier] = sym_identifier,
  [anon_sym_POUND] = anon_sym_POUND,
  [aux_sym_comment_token1] = aux_sym_comment_token1,
  [anon_sym_SLASH_SLASH] = anon_sym_SLASH_SLASH,
  [anon_sym_SLASH_STAR] = anon_sym_SLASH_STAR,
  [aux_sym_comment_token2] = aux_sym_comment_token2,
  [sym_runbook] = sym_runbook,
  [sym__statement] = sym__statement,
  [sym_addon_block] = sym_addon_block,
  [sym_signer_block] = sym_signer_block,
  [sym_action_block] = sym_action_block,
  [sym_output_block] = sym_output_block,
  [sym_variable_declaration] = sym_variable_declaration,
  [sym_input_declaration] = sym_input_declaration,
  [sym_import_statement] = sym_import_statement,
  [sym_block] = sym_block,
  [sym_attribute] = sym_attribute,
  [sym__expression] = sym__expression,
  [sym_string] = sym_string,
  [sym_number] = sym_number,
  [sym_boolean] = sym_boolean,
  [sym_array] = sym_array,
  [sym_object] = sym_object,
  [sym_object_field] = sym_object_field,
  [sym_reference] = sym_reference,
  [sym_index_access] = sym_index_access,
  [sym_function_call] = sym_function_call,
  [sym_binary_expression] = sym_binary_expression,
  [sym_comment] = sym_comment,
  [aux_sym_runbook_repeat1] = aux_sym_runbook_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
  [aux_sym_array_repeat1] = aux_sym_array_repeat1,
  [aux_sym_object_repeat1] = aux_sym_object_repeat1,
  [aux_sym_reference_repeat1] = aux_sym_reference_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_addon] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_signer] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_action] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_output] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_variable] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_input] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DQUOTE_DQUOTE_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_token3] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_number_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_number_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_number_token3] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [sym_null] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_comment_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_SLASH_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_STAR] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_comment_token2] = {
    .visible = false,
    .named = false,
  },
  [sym_runbook] = {
    .visible = true,
    .named = true,
  },
  [sym__statement] = {
    .visible = false,
    .named = true,
  },
  [sym_addon_block] = {
    .visible = true,
    .named = true,
  },
  [sym_signer_block] = {
    .visible = true,
    .named = true,
  },
  [sym_action_block] = {
    .visible = true,
    .named = true,
  },
  [sym_output_block] = {
    .visible = true,
    .named = true,
  },
  [sym_variable_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym_input_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym_import_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute] = {
    .visible = true,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_boolean] = {
    .visible = true,
    .named = true,
  },
  [sym_array] = {
    .visible = true,
    .named = true,
  },
  [sym_object] = {
    .visible = true,
    .named = true,
  },
  [sym_object_field] = {
    .visible = true,
    .named = true,
  },
  [sym_reference] = {
    .visible = true,
    .named = true,
  },
  [sym_index_access] = {
    .visible = true,
    .named = true,
  },
  [sym_function_call] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_runbook_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_array_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_object_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_reference_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_arguments = 1,
  field_config = 2,
  field_key = 3,
  field_name = 4,
  field_network = 5,
  field_path = 6,
  field_type = 7,
  field_value = 8,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_arguments] = "arguments",
  [field_config] = "config",
  [field_key] = "key",
  [field_name] = "name",
  [field_network] = "network",
  [field_path] = "path",
  [field_type] = "type",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 2},
  [3] = {.index = 3, .length = 2},
  [4] = {.index = 5, .length = 3},
  [5] = {.index = 8, .length = 2},
  [6] = {.index = 10, .length = 2},
  [7] = {.index = 12, .length = 1},
  [8] = {.index = 13, .length = 2},
  [9] = {.index = 15, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_path, 1},
  [1] =
    {field_config, 2},
    {field_network, 1},
  [3] =
    {field_config, 2},
    {field_name, 1},
  [5] =
    {field_config, 3},
    {field_name, 1},
    {field_type, 2},
  [8] =
    {field_name, 1},
    {field_value, 3},
  [10] =
    {field_key, 0},
    {field_value, 2},
  [12] =
    {field_name, 0},
  [13] =
    {field_arguments, 2},
    {field_name, 0},
  [15] =
    {field_arguments, 2},
    {field_arguments, 3},
    {field_name, 0},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 2,
  [4] = 4,
  [5] = 5,
  [6] = 4,
  [7] = 5,
  [8] = 8,
  [9] = 8,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 10,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 13,
  [19] = 19,
  [20] = 12,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 21,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 25,
  [56] = 24,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 23,
  [61] = 22,
  [62] = 59,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 65,
  [67] = 67,
  [68] = 27,
  [69] = 69,
  [70] = 70,
  [71] = 67,
  [72] = 26,
  [73] = 70,
  [74] = 69,
  [75] = 35,
  [76] = 44,
  [77] = 77,
  [78] = 39,
  [79] = 79,
  [80] = 80,
  [81] = 45,
  [82] = 38,
  [83] = 43,
  [84] = 41,
  [85] = 42,
  [86] = 40,
  [87] = 17,
  [88] = 30,
  [89] = 34,
  [90] = 36,
  [91] = 31,
  [92] = 37,
  [93] = 32,
  [94] = 33,
  [95] = 95,
  [96] = 95,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 110,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 114,
  [117] = 113,
  [118] = 111,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 119,
  [128] = 128,
  [129] = 129,
  [130] = 129,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 135,
  [136] = 128,
  [137] = 137,
  [138] = 131,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 134,
  [145] = 145,
  [146] = 146,
  [147] = 137,
  [148] = 141,
  [149] = 135,
  [150] = 132,
  [151] = 151,
  [152] = 152,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(47);
      if (lookahead == '"') ADVANCE(66);
      if (lookahead == '#') ADVANCE(141);
      if (lookahead == '\'') ADVANCE(70);
      if (lookahead == '(') ADVANCE(91);
      if (lookahead == ')') ADVANCE(92);
      if (lookahead == '*') ADVANCE(93);
      if (lookahead == '+') ADVANCE(95);
      if (lookahead == ',') ADVANCE(87);
      if (lookahead == '-') ADVANCE(96);
      if (lookahead == '.') ADVANCE(90);
      if (lookahead == '/') ADVANCE(94);
      if (lookahead == '0') ADVANCE(81);
      if (lookahead == ':') ADVANCE(89);
      if (lookahead == '=') ADVANCE(60);
      if (lookahead == '[') ADVANCE(86);
      if (lookahead == ']') ADVANCE(88);
      if (lookahead == 'a') ADVANCE(101);
      if (lookahead == 'f') ADVANCE(97);
      if (lookahead == 'i') ADVANCE(115);
      if (lookahead == 'n') ADVANCE(137);
      if (lookahead == 'o') ADVANCE(136);
      if (lookahead == 's') ADVANCE(108);
      if (lookahead == 't') ADVANCE(126);
      if (lookahead == 'v') ADVANCE(99);
      if (lookahead == '{') ADVANCE(63);
      if (lookahead == '}') ADVANCE(64);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(82);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(66);
      if (lookahead == '#') ADVANCE(141);
      if (lookahead == '\'') ADVANCE(70);
      if (lookahead == '(') ADVANCE(91);
      if (lookahead == '*') ADVANCE(93);
      if (lookahead == '+') ADVANCE(95);
      if (lookahead == ',') ADVANCE(87);
      if (lookahead == '-') ADVANCE(96);
      if (lookahead == '.') ADVANCE(90);
      if (lookahead == '/') ADVANCE(94);
      if (lookahead == '[') ADVANCE(86);
      if (lookahead == '}') ADVANCE(64);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(66);
      if (lookahead == '#') ADVANCE(141);
      if (lookahead == '\'') ADVANCE(70);
      if (lookahead == ')') ADVANCE(92);
      if (lookahead == ',') ADVANCE(87);
      if (lookahead == '/') ADVANCE(8);
      if (lookahead == '0') ADVANCE(81);
      if (lookahead == '[') ADVANCE(86);
      if (lookahead == ']') ADVANCE(88);
      if (lookahead == 'f') ADVANCE(97);
      if (lookahead == 'n') ADVANCE(137);
      if (lookahead == 't') ADVANCE(126);
      if (lookahead == '{') ADVANCE(63);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(2)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(82);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 3:
      if (lookahead == '"') ADVANCE(74);
      END_STATE();
    case 4:
      if (lookahead == '"') ADVANCE(5);
      if (lookahead == '#') ADVANCE(141);
      if (lookahead == '/') ADVANCE(77);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(76);
      if (lookahead != 0) ADVANCE(75);
      END_STATE();
    case 5:
      if (lookahead == '"') ADVANCE(78);
      if (lookahead != 0) ADVANCE(75);
      END_STATE();
    case 6:
      if (lookahead == '"') ADVANCE(65);
      if (lookahead == '#') ADVANCE(141);
      if (lookahead == '/') ADVANCE(8);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(6)
      END_STATE();
    case 7:
      if (lookahead == '#') ADVANCE(142);
      if (lookahead == '*') ADVANCE(159);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(7);
      if (lookahead != 0) ADVANCE(9);
      END_STATE();
    case 8:
      if (lookahead == '*') ADVANCE(154);
      if (lookahead == '/') ADVANCE(149);
      END_STATE();
    case 9:
      if (lookahead == '*') ADVANCE(159);
      if (lookahead != 0) ADVANCE(9);
      END_STATE();
    case 10:
      if (lookahead == '*') ADVANCE(155);
      if (lookahead == '/') ADVANCE(150);
      if (lookahead != 0) ADVANCE(9);
      END_STATE();
    case 11:
      if (lookahead == 'a') ADVANCE(34);
      END_STATE();
    case 12:
      if (lookahead == 'a') ADVANCE(13);
      END_STATE();
    case 13:
      if (lookahead == 'b') ADVANCE(22);
      END_STATE();
    case 14:
      if (lookahead == 'c') ADVANCE(39);
      if (lookahead == 'd') ADVANCE(15);
      END_STATE();
    case 15:
      if (lookahead == 'd') ADVANCE(28);
      END_STATE();
    case 16:
      if (lookahead == 'e') ADVANCE(56);
      END_STATE();
    case 17:
      if (lookahead == 'e') ADVANCE(33);
      END_STATE();
    case 18:
      if (lookahead == 'g') ADVANCE(24);
      END_STATE();
    case 19:
      if (lookahead == 'i') ADVANCE(18);
      END_STATE();
    case 20:
      if (lookahead == 'i') ADVANCE(12);
      END_STATE();
    case 21:
      if (lookahead == 'i') ADVANCE(29);
      END_STATE();
    case 22:
      if (lookahead == 'l') ADVANCE(16);
      END_STATE();
    case 23:
      if (lookahead == 'm') ADVANCE(30);
      if (lookahead == 'n') ADVANCE(31);
      END_STATE();
    case 24:
      if (lookahead == 'n') ADVANCE(17);
      END_STATE();
    case 25:
      if (lookahead == 'n') ADVANCE(48);
      END_STATE();
    case 26:
      if (lookahead == 'n') ADVANCE(52);
      END_STATE();
    case 27:
      if (lookahead == 'o') ADVANCE(35);
      END_STATE();
    case 28:
      if (lookahead == 'o') ADVANCE(25);
      END_STATE();
    case 29:
      if (lookahead == 'o') ADVANCE(26);
      END_STATE();
    case 30:
      if (lookahead == 'p') ADVANCE(27);
      END_STATE();
    case 31:
      if (lookahead == 'p') ADVANCE(42);
      END_STATE();
    case 32:
      if (lookahead == 'p') ADVANCE(43);
      END_STATE();
    case 33:
      if (lookahead == 'r') ADVANCE(50);
      END_STATE();
    case 34:
      if (lookahead == 'r') ADVANCE(20);
      END_STATE();
    case 35:
      if (lookahead == 'r') ADVANCE(37);
      END_STATE();
    case 36:
      if (lookahead == 't') ADVANCE(58);
      END_STATE();
    case 37:
      if (lookahead == 't') ADVANCE(61);
      END_STATE();
    case 38:
      if (lookahead == 't') ADVANCE(54);
      END_STATE();
    case 39:
      if (lookahead == 't') ADVANCE(21);
      END_STATE();
    case 40:
      if (lookahead == 't') ADVANCE(32);
      END_STATE();
    case 41:
      if (lookahead == 'u') ADVANCE(40);
      END_STATE();
    case 42:
      if (lookahead == 'u') ADVANCE(36);
      END_STATE();
    case 43:
      if (lookahead == 'u') ADVANCE(38);
      END_STATE();
    case 44:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(80);
      END_STATE();
    case 45:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(79);
      END_STATE();
    case 46:
      if (eof) ADVANCE(47);
      if (lookahead == '"') ADVANCE(66);
      if (lookahead == '#') ADVANCE(141);
      if (lookahead == '\'') ADVANCE(70);
      if (lookahead == '(') ADVANCE(91);
      if (lookahead == ')') ADVANCE(92);
      if (lookahead == '*') ADVANCE(93);
      if (lookahead == '+') ADVANCE(95);
      if (lookahead == ',') ADVANCE(87);
      if (lookahead == '-') ADVANCE(96);
      if (lookahead == '.') ADVANCE(90);
      if (lookahead == '/') ADVANCE(94);
      if (lookahead == ':') ADVANCE(89);
      if (lookahead == '=') ADVANCE(60);
      if (lookahead == '[') ADVANCE(86);
      if (lookahead == ']') ADVANCE(88);
      if (lookahead == 'a') ADVANCE(14);
      if (lookahead == 'i') ADVANCE(23);
      if (lookahead == 'o') ADVANCE(41);
      if (lookahead == 's') ADVANCE(19);
      if (lookahead == 'v') ADVANCE(11);
      if (lookahead == '{') ADVANCE(63);
      if (lookahead == '}') ADVANCE(64);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(46)
      END_STATE();
    case 47:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_addon);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_addon);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_signer);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_signer);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(anon_sym_action);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(anon_sym_action);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_output);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(anon_sym_output);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_variable);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(anon_sym_variable);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(anon_sym_input);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(anon_sym_input);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(anon_sym_import);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      if (lookahead == '"') ADVANCE(3);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '#') ADVANCE(144);
      if (lookahead == '/') ADVANCE(68);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(67);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(69);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '*') ADVANCE(157);
      if (lookahead == '/') ADVANCE(152);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(69);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(69);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '#') ADVANCE(145);
      if (lookahead == '/') ADVANCE(72);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(71);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(73);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '*') ADVANCE(158);
      if (lookahead == '/') ADVANCE(153);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(73);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(73);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(anon_sym_DQUOTE_DQUOTE_DQUOTE);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(aux_sym_string_token3);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(aux_sym_string_token3);
      if (lookahead == '"') ADVANCE(5);
      if (lookahead == '#') ADVANCE(141);
      if (lookahead == '/') ADVANCE(77);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(76);
      if (lookahead != 0) ADVANCE(75);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(aux_sym_string_token3);
      if (lookahead == '*') ADVANCE(154);
      if (lookahead == '/') ADVANCE(149);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(aux_sym_string_token3);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(78);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(79);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(aux_sym_number_token2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(80);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(aux_sym_number_token3);
      if (lookahead == '.') ADVANCE(44);
      if (lookahead == 'x') ADVANCE(45);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(82);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(aux_sym_number_token3);
      if (lookahead == '.') ADVANCE(44);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(82);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(anon_sym_true);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_false);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(sym_null);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(154);
      if (lookahead == '/') ADVANCE(149);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(111);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(100);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(127);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'b') ADVANCE(114);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(133);
      if (lookahead == 'd') ADVANCE(102);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'd') ADVANCE(119);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(83);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(84);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(57);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(125);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(118);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(107);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(98);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(121);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(129);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(85);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(112);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(105);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(122);
      if (lookahead == 'n') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(49);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(53);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(106);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(116);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(128);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(117);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(120);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 123:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(138);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 124:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(139);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(51);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(135);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(109);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(131);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(104);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(59);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 131:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(62);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(55);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 133:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(124);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(103);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 136:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(134);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(113);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(130);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 139:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(132);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 140:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(140);
      END_STATE();
    case 141:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 142:
      ACCEPT_TOKEN(anon_sym_POUND);
      if (lookahead == '*') ADVANCE(159);
      if (lookahead != 0) ADVANCE(9);
      END_STATE();
    case 143:
      ACCEPT_TOKEN(anon_sym_POUND);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(148);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(anon_sym_POUND);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(69);
      END_STATE();
    case 145:
      ACCEPT_TOKEN(anon_sym_POUND);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(73);
      END_STATE();
    case 146:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (lookahead == '#') ADVANCE(143);
      if (lookahead == '/') ADVANCE(147);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(146);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(148);
      END_STATE();
    case 147:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (lookahead == '*') ADVANCE(156);
      if (lookahead == '/') ADVANCE(151);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(148);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(148);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      if (lookahead == '*') ADVANCE(159);
      if (lookahead != 0) ADVANCE(9);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(148);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(69);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(73);
      END_STATE();
    case 154:
      ACCEPT_TOKEN(anon_sym_SLASH_STAR);
      END_STATE();
    case 155:
      ACCEPT_TOKEN(anon_sym_SLASH_STAR);
      if (lookahead == '*') ADVANCE(159);
      if (lookahead != 0 &&
          lookahead != '/') ADVANCE(9);
      END_STATE();
    case 156:
      ACCEPT_TOKEN(anon_sym_SLASH_STAR);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(148);
      END_STATE();
    case 157:
      ACCEPT_TOKEN(anon_sym_SLASH_STAR);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(69);
      END_STATE();
    case 158:
      ACCEPT_TOKEN(anon_sym_SLASH_STAR);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(73);
      END_STATE();
    case 159:
      ACCEPT_TOKEN(aux_sym_comment_token2);
      if (lookahead == '*') ADVANCE(159);
      if (lookahead != 0 &&
          lookahead != '/') ADVANCE(9);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 46},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 2},
  [6] = {.lex_state = 2},
  [7] = {.lex_state = 2},
  [8] = {.lex_state = 2},
  [9] = {.lex_state = 2},
  [10] = {.lex_state = 2},
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 2},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 2},
  [16] = {.lex_state = 2},
  [17] = {.lex_state = 46},
  [18] = {.lex_state = 2},
  [19] = {.lex_state = 2},
  [20] = {.lex_state = 2},
  [21] = {.lex_state = 46},
  [22] = {.lex_state = 46},
  [23] = {.lex_state = 46},
  [24] = {.lex_state = 46},
  [25] = {.lex_state = 46},
  [26] = {.lex_state = 46},
  [27] = {.lex_state = 46},
  [28] = {.lex_state = 46},
  [29] = {.lex_state = 46},
  [30] = {.lex_state = 46},
  [31] = {.lex_state = 46},
  [32] = {.lex_state = 46},
  [33] = {.lex_state = 46},
  [34] = {.lex_state = 46},
  [35] = {.lex_state = 46},
  [36] = {.lex_state = 46},
  [37] = {.lex_state = 46},
  [38] = {.lex_state = 46},
  [39] = {.lex_state = 46},
  [40] = {.lex_state = 46},
  [41] = {.lex_state = 46},
  [42] = {.lex_state = 46},
  [43] = {.lex_state = 46},
  [44] = {.lex_state = 46},
  [45] = {.lex_state = 46},
  [46] = {.lex_state = 46},
  [47] = {.lex_state = 1},
  [48] = {.lex_state = 46},
  [49] = {.lex_state = 46},
  [50] = {.lex_state = 46},
  [51] = {.lex_state = 46},
  [52] = {.lex_state = 46},
  [53] = {.lex_state = 46},
  [54] = {.lex_state = 46},
  [55] = {.lex_state = 1},
  [56] = {.lex_state = 1},
  [57] = {.lex_state = 46},
  [58] = {.lex_state = 46},
  [59] = {.lex_state = 1},
  [60] = {.lex_state = 1},
  [61] = {.lex_state = 1},
  [62] = {.lex_state = 1},
  [63] = {.lex_state = 46},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 1},
  [66] = {.lex_state = 1},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 1},
  [69] = {.lex_state = 1},
  [70] = {.lex_state = 0},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 1},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 1},
  [75] = {.lex_state = 1},
  [76] = {.lex_state = 1},
  [77] = {.lex_state = 1},
  [78] = {.lex_state = 1},
  [79] = {.lex_state = 1},
  [80] = {.lex_state = 0},
  [81] = {.lex_state = 1},
  [82] = {.lex_state = 1},
  [83] = {.lex_state = 1},
  [84] = {.lex_state = 1},
  [85] = {.lex_state = 1},
  [86] = {.lex_state = 1},
  [87] = {.lex_state = 1},
  [88] = {.lex_state = 1},
  [89] = {.lex_state = 1},
  [90] = {.lex_state = 1},
  [91] = {.lex_state = 1},
  [92] = {.lex_state = 1},
  [93] = {.lex_state = 1},
  [94] = {.lex_state = 1},
  [95] = {.lex_state = 0},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 1},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 1},
  [104] = {.lex_state = 1},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 0},
  [119] = {.lex_state = 1},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 0},
  [122] = {.lex_state = 0},
  [123] = {.lex_state = 1},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 0},
  [126] = {.lex_state = 0},
  [127] = {.lex_state = 1},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 0},
  [130] = {.lex_state = 0},
  [131] = {.lex_state = 0},
  [132] = {.lex_state = 71},
  [133] = {.lex_state = 0},
  [134] = {.lex_state = 0},
  [135] = {.lex_state = 67},
  [136] = {.lex_state = 0},
  [137] = {.lex_state = 4},
  [138] = {.lex_state = 0},
  [139] = {.lex_state = 0},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 6},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 146},
  [144] = {.lex_state = 0},
  [145] = {.lex_state = 7},
  [146] = {.lex_state = 0},
  [147] = {.lex_state = 4},
  [148] = {.lex_state = 6},
  [149] = {.lex_state = 67},
  [150] = {.lex_state = 71},
  [151] = {(TSStateId)(-1)},
  [152] = {(TSStateId)(-1)},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [sym_comment] = STATE(0),
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_addon] = ACTIONS(1),
    [anon_sym_signer] = ACTIONS(1),
    [anon_sym_action] = ACTIONS(1),
    [anon_sym_output] = ACTIONS(1),
    [anon_sym_variable] = ACTIONS(1),
    [anon_sym_input] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE_DQUOTE_DQUOTE] = ACTIONS(1),
    [aux_sym_number_token1] = ACTIONS(1),
    [aux_sym_number_token2] = ACTIONS(1),
    [aux_sym_number_token3] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [sym_null] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_POUND] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
    [anon_sym_SLASH_STAR] = ACTIONS(5),
  },
  [1] = {
    [sym_runbook] = STATE(142),
    [sym__statement] = STATE(48),
    [sym_addon_block] = STATE(51),
    [sym_signer_block] = STATE(51),
    [sym_action_block] = STATE(51),
    [sym_output_block] = STATE(51),
    [sym_variable_declaration] = STATE(51),
    [sym_input_declaration] = STATE(51),
    [sym_import_statement] = STATE(51),
    [sym_comment] = STATE(1),
    [aux_sym_runbook_repeat1] = STATE(28),
    [ts_builtin_sym_end] = ACTIONS(7),
    [anon_sym_addon] = ACTIONS(9),
    [anon_sym_signer] = ACTIONS(11),
    [anon_sym_action] = ACTIONS(13),
    [anon_sym_output] = ACTIONS(15),
    [anon_sym_variable] = ACTIONS(17),
    [anon_sym_input] = ACTIONS(19),
    [anon_sym_import] = ACTIONS(21),
    [anon_sym_POUND] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
    [anon_sym_SLASH_STAR] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 18,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(41), 1,
      anon_sym_COMMA,
    ACTIONS(43), 1,
      anon_sym_RBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    STATE(2), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(70), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [65] = 18,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    ACTIONS(47), 1,
      anon_sym_COMMA,
    ACTIONS(49), 1,
      anon_sym_RBRACK,
    STATE(3), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(73), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [130] = 17,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    ACTIONS(51), 1,
      anon_sym_RBRACK,
    STATE(4), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(64), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [192] = 17,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    ACTIONS(53), 1,
      anon_sym_RBRACK,
    STATE(5), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(64), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [254] = 17,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    ACTIONS(55), 1,
      anon_sym_RBRACK,
    STATE(6), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(64), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [316] = 17,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    ACTIONS(57), 1,
      anon_sym_RBRACK,
    STATE(7), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(64), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [378] = 17,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    ACTIONS(59), 1,
      anon_sym_RPAREN,
    STATE(8), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(67), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [440] = 17,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    ACTIONS(61), 1,
      anon_sym_RPAREN,
    STATE(9), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(71), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [502] = 16,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(63), 1,
      anon_sym_LBRACE,
    ACTIONS(65), 1,
      anon_sym_DQUOTE,
    ACTIONS(67), 1,
      anon_sym_SQUOTE,
    ACTIONS(69), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(73), 1,
      aux_sym_number_token3,
    ACTIONS(77), 1,
      sym_null,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      sym_identifier,
    STATE(10), 1,
      sym_comment,
    STATE(55), 1,
      sym_index_access,
    STATE(83), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(71), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(75), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(93), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [561] = 16,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    STATE(11), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(80), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [620] = 16,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    STATE(12), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(95), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [679] = 16,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    STATE(13), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(41), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [738] = 16,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    STATE(14), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(43), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [797] = 16,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    STATE(15), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(64), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [856] = 16,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    STATE(16), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(46), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [915] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(17), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(85), 2,
      anon_sym_DQUOTE,
      anon_sym_SLASH,
    ACTIONS(83), 20,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_EQ,
      anon_sym_import,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [952] = 16,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(63), 1,
      anon_sym_LBRACE,
    ACTIONS(65), 1,
      anon_sym_DQUOTE,
    ACTIONS(67), 1,
      anon_sym_SQUOTE,
    ACTIONS(69), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(73), 1,
      aux_sym_number_token3,
    ACTIONS(77), 1,
      sym_null,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      sym_identifier,
    STATE(18), 1,
      sym_comment,
    STATE(55), 1,
      sym_index_access,
    STATE(84), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(71), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(75), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(93), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [1011] = 16,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(63), 1,
      anon_sym_LBRACE,
    ACTIONS(65), 1,
      anon_sym_DQUOTE,
    ACTIONS(67), 1,
      anon_sym_SQUOTE,
    ACTIONS(69), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(73), 1,
      aux_sym_number_token3,
    ACTIONS(77), 1,
      sym_null,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      sym_identifier,
    STATE(19), 1,
      sym_comment,
    STATE(55), 1,
      sym_index_access,
    STATE(77), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(71), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(75), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(93), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [1070] = 16,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(23), 1,
      anon_sym_LBRACE,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(33), 1,
      aux_sym_number_token3,
    ACTIONS(37), 1,
      sym_null,
    ACTIONS(39), 1,
      anon_sym_LBRACK,
    ACTIONS(45), 1,
      sym_identifier,
    STATE(20), 1,
      sym_comment,
    STATE(25), 1,
      sym_index_access,
    STATE(96), 1,
      sym__expression,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(31), 2,
      aux_sym_number_token1,
      aux_sym_number_token2,
    ACTIONS(35), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(32), 8,
      sym_string,
      sym_number,
      sym_boolean,
      sym_array,
      sym_object,
      sym_reference,
      sym_function_call,
      sym_binary_expression,
  [1129] = 9,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(89), 1,
      anon_sym_LBRACK,
    ACTIONS(91), 1,
      anon_sym_DOT,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_SLASH,
    STATE(21), 1,
      sym_comment,
    STATE(24), 1,
      aux_sym_reference_repeat1,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(87), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1172] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(99), 1,
      anon_sym_DOT,
    ACTIONS(102), 1,
      anon_sym_SLASH,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    STATE(22), 2,
      sym_comment,
      aux_sym_reference_repeat1,
    ACTIONS(97), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1207] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(89), 1,
      anon_sym_LBRACK,
    ACTIONS(102), 1,
      anon_sym_SLASH,
    STATE(23), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(97), 16,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1242] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(91), 1,
      anon_sym_DOT,
    ACTIONS(106), 1,
      anon_sym_SLASH,
    STATE(22), 1,
      aux_sym_reference_repeat1,
    STATE(24), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(104), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1279] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(91), 1,
      anon_sym_DOT,
    ACTIONS(95), 1,
      anon_sym_SLASH,
    STATE(24), 1,
      aux_sym_reference_repeat1,
    STATE(25), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(87), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1316] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(110), 1,
      anon_sym_SLASH,
    STATE(26), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(108), 16,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1348] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(102), 1,
      anon_sym_SLASH,
    STATE(27), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(97), 16,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1380] = 14,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(9), 1,
      anon_sym_addon,
    ACTIONS(11), 1,
      anon_sym_signer,
    ACTIONS(13), 1,
      anon_sym_action,
    ACTIONS(15), 1,
      anon_sym_output,
    ACTIONS(17), 1,
      anon_sym_variable,
    ACTIONS(19), 1,
      anon_sym_input,
    ACTIONS(21), 1,
      anon_sym_import,
    ACTIONS(112), 1,
      ts_builtin_sym_end,
    STATE(28), 1,
      sym_comment,
    STATE(29), 1,
      aux_sym_runbook_repeat1,
    STATE(48), 1,
      sym__statement,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    STATE(51), 7,
      sym_addon_block,
      sym_signer_block,
      sym_action_block,
      sym_output_block,
      sym_variable_declaration,
      sym_input_declaration,
      sym_import_statement,
  [1430] = 13,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(114), 1,
      ts_builtin_sym_end,
    ACTIONS(116), 1,
      anon_sym_addon,
    ACTIONS(119), 1,
      anon_sym_signer,
    ACTIONS(122), 1,
      anon_sym_action,
    ACTIONS(125), 1,
      anon_sym_output,
    ACTIONS(128), 1,
      anon_sym_variable,
    ACTIONS(131), 1,
      anon_sym_input,
    ACTIONS(134), 1,
      anon_sym_import,
    STATE(48), 1,
      sym__statement,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    STATE(29), 2,
      sym_comment,
      aux_sym_runbook_repeat1,
    STATE(51), 7,
      sym_addon_block,
      sym_signer_block,
      sym_action_block,
      sym_output_block,
      sym_variable_declaration,
      sym_input_declaration,
      sym_import_statement,
  [1478] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(139), 1,
      anon_sym_SLASH,
    STATE(30), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(137), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1509] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(143), 1,
      anon_sym_SLASH,
    STATE(31), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(141), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1540] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    STATE(32), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(145), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1571] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(151), 1,
      anon_sym_SLASH,
    STATE(33), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(149), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1602] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    STATE(34), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(153), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1633] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(159), 1,
      anon_sym_SLASH,
    STATE(35), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(157), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1664] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(163), 1,
      anon_sym_SLASH,
    STATE(36), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(161), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1695] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(167), 1,
      anon_sym_SLASH,
    STATE(37), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(165), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1726] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(171), 1,
      anon_sym_SLASH,
    STATE(38), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(169), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1757] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(175), 1,
      anon_sym_SLASH,
    STATE(39), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(173), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1788] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(179), 1,
      anon_sym_SLASH,
    STATE(40), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(177), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1819] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(183), 1,
      anon_sym_SLASH,
    STATE(41), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(181), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1850] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(187), 1,
      anon_sym_SLASH,
    STATE(42), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(185), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1881] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(189), 1,
      anon_sym_STAR,
    ACTIONS(191), 1,
      anon_sym_SLASH,
    STATE(43), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(181), 14,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1914] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(195), 1,
      anon_sym_SLASH,
    STATE(44), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(193), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1945] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(199), 1,
      anon_sym_SLASH,
    STATE(45), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(197), 15,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [1976] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(189), 1,
      anon_sym_STAR,
    ACTIONS(191), 1,
      anon_sym_SLASH,
    STATE(46), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(203), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(201), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2007] = 9,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(95), 1,
      anon_sym_SLASH,
    ACTIONS(205), 1,
      anon_sym_LBRACK,
    ACTIONS(207), 1,
      anon_sym_DOT,
    ACTIONS(209), 1,
      anon_sym_LPAREN,
    STATE(47), 1,
      sym_comment,
    STATE(56), 1,
      aux_sym_reference_repeat1,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(87), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2040] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(48), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(211), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2061] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(49), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(213), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2082] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(50), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(215), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2103] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(51), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(217), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2124] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(52), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(219), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2145] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(53), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(221), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2166] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(54), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(223), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2187] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(95), 1,
      anon_sym_SLASH,
    ACTIONS(207), 1,
      anon_sym_DOT,
    STATE(55), 1,
      sym_comment,
    STATE(56), 1,
      aux_sym_reference_repeat1,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(87), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2214] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(106), 1,
      anon_sym_SLASH,
    ACTIONS(207), 1,
      anon_sym_DOT,
    STATE(56), 1,
      sym_comment,
    STATE(61), 1,
      aux_sym_reference_repeat1,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(104), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2241] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(57), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(225), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2262] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(58), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(227), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2283] = 11,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(229), 1,
      anon_sym_RBRACE,
    ACTIONS(231), 1,
      anon_sym_COMMA,
    ACTIONS(233), 1,
      sym_identifier,
    STATE(59), 1,
      sym_comment,
    STATE(114), 1,
      sym_object_field,
    STATE(140), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [2318] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(102), 1,
      anon_sym_SLASH,
    ACTIONS(205), 1,
      anon_sym_LBRACK,
    STATE(60), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(97), 6,
      anon_sym_RBRACE,
      anon_sym_DOT,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2343] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(102), 1,
      anon_sym_SLASH,
    ACTIONS(235), 1,
      anon_sym_DOT,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    STATE(61), 2,
      sym_comment,
      aux_sym_reference_repeat1,
    ACTIONS(97), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2368] = 11,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(233), 1,
      sym_identifier,
    ACTIONS(238), 1,
      anon_sym_RBRACE,
    ACTIONS(240), 1,
      anon_sym_COMMA,
    STATE(62), 1,
      sym_comment,
    STATE(116), 1,
      sym_object_field,
    STATE(140), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [2403] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(63), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(242), 8,
      ts_builtin_sym_end,
      anon_sym_addon,
      anon_sym_signer,
      anon_sym_action,
      anon_sym_output,
      anon_sym_variable,
      anon_sym_input,
      anon_sym_import,
  [2424] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(189), 1,
      anon_sym_STAR,
    ACTIONS(191), 1,
      anon_sym_SLASH,
    STATE(64), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(203), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(244), 3,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
  [2450] = 10,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(233), 1,
      sym_identifier,
    ACTIONS(246), 1,
      anon_sym_RBRACE,
    STATE(65), 1,
      sym_comment,
    STATE(120), 1,
      sym_object_field,
    STATE(140), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [2482] = 10,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(233), 1,
      sym_identifier,
    ACTIONS(248), 1,
      anon_sym_RBRACE,
    STATE(66), 1,
      sym_comment,
    STATE(120), 1,
      sym_object_field,
    STATE(140), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [2514] = 9,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(189), 1,
      anon_sym_STAR,
    ACTIONS(191), 1,
      anon_sym_SLASH,
    ACTIONS(250), 1,
      anon_sym_COMMA,
    ACTIONS(252), 1,
      anon_sym_RPAREN,
    STATE(67), 1,
      sym_comment,
    STATE(117), 1,
      aux_sym_array_repeat1,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(203), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [2544] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(102), 1,
      anon_sym_SLASH,
    STATE(68), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(97), 6,
      anon_sym_RBRACE,
      anon_sym_DOT,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2566] = 10,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(233), 1,
      sym_identifier,
    ACTIONS(254), 1,
      anon_sym_RBRACE,
    STATE(69), 1,
      sym_comment,
    STATE(120), 1,
      sym_object_field,
    STATE(140), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [2598] = 9,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(189), 1,
      anon_sym_STAR,
    ACTIONS(191), 1,
      anon_sym_SLASH,
    ACTIONS(256), 1,
      anon_sym_COMMA,
    ACTIONS(258), 1,
      anon_sym_RBRACK,
    STATE(70), 1,
      sym_comment,
    STATE(112), 1,
      aux_sym_array_repeat1,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(203), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [2628] = 9,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(189), 1,
      anon_sym_STAR,
    ACTIONS(191), 1,
      anon_sym_SLASH,
    ACTIONS(250), 1,
      anon_sym_COMMA,
    ACTIONS(260), 1,
      anon_sym_RPAREN,
    STATE(71), 1,
      sym_comment,
    STATE(113), 1,
      aux_sym_array_repeat1,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(203), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [2658] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(110), 1,
      anon_sym_SLASH,
    STATE(72), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(108), 6,
      anon_sym_RBRACE,
      anon_sym_DOT,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2680] = 9,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(189), 1,
      anon_sym_STAR,
    ACTIONS(191), 1,
      anon_sym_SLASH,
    ACTIONS(262), 1,
      anon_sym_COMMA,
    ACTIONS(264), 1,
      anon_sym_RBRACK,
    STATE(73), 1,
      sym_comment,
    STATE(110), 1,
      aux_sym_array_repeat1,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(203), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [2710] = 10,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(233), 1,
      sym_identifier,
    ACTIONS(266), 1,
      anon_sym_RBRACE,
    STATE(74), 1,
      sym_comment,
    STATE(120), 1,
      sym_object_field,
    STATE(140), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [2742] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(159), 1,
      anon_sym_SLASH,
    STATE(75), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(157), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2763] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(195), 1,
      anon_sym_SLASH,
    STATE(76), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(193), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2784] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(270), 1,
      anon_sym_STAR,
    ACTIONS(272), 1,
      anon_sym_SLASH,
    STATE(77), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(268), 2,
      anon_sym_RBRACE,
      sym_identifier,
    ACTIONS(274), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [2809] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(175), 1,
      anon_sym_SLASH,
    STATE(78), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(173), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2830] = 9,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    ACTIONS(233), 1,
      sym_identifier,
    STATE(79), 1,
      sym_comment,
    STATE(120), 1,
      sym_object_field,
    STATE(140), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [2859] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(189), 1,
      anon_sym_STAR,
    ACTIONS(191), 1,
      anon_sym_SLASH,
    STATE(80), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(203), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(276), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [2884] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(199), 1,
      anon_sym_SLASH,
    STATE(81), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(197), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2905] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(171), 1,
      anon_sym_SLASH,
    STATE(82), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(169), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2926] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(270), 1,
      anon_sym_STAR,
    ACTIONS(272), 1,
      anon_sym_SLASH,
    STATE(83), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(181), 4,
      anon_sym_RBRACE,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2949] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(183), 1,
      anon_sym_SLASH,
    STATE(84), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(181), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2970] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(187), 1,
      anon_sym_SLASH,
    STATE(85), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(185), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [2991] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(179), 1,
      anon_sym_SLASH,
    STATE(86), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(177), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [3012] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(85), 1,
      anon_sym_SLASH,
    STATE(87), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(83), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [3033] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(139), 1,
      anon_sym_SLASH,
    STATE(88), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(137), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [3054] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(155), 1,
      anon_sym_SLASH,
    STATE(89), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(153), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [3075] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(163), 1,
      anon_sym_SLASH,
    STATE(90), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(161), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [3096] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(143), 1,
      anon_sym_SLASH,
    STATE(91), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(141), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [3117] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(167), 1,
      anon_sym_SLASH,
    STATE(92), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(165), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [3138] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(147), 1,
      anon_sym_SLASH,
    STATE(93), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(145), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [3159] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(151), 1,
      anon_sym_SLASH,
    STATE(94), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(149), 5,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_identifier,
  [3180] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(189), 1,
      anon_sym_STAR,
    ACTIONS(191), 1,
      anon_sym_SLASH,
    ACTIONS(278), 1,
      anon_sym_RBRACK,
    STATE(95), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(203), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [3204] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(189), 1,
      anon_sym_STAR,
    ACTIONS(191), 1,
      anon_sym_SLASH,
    ACTIONS(280), 1,
      anon_sym_RBRACK,
    STATE(96), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(203), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [3228] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(282), 1,
      anon_sym_RBRACE,
    ACTIONS(284), 1,
      sym_identifier,
    STATE(97), 1,
      sym_comment,
    STATE(103), 1,
      aux_sym_block_repeat1,
    STATE(123), 1,
      sym_attribute,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3251] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(98), 1,
      sym_comment,
    STATE(121), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3274] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(99), 1,
      sym_comment,
    STATE(124), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3297] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(50), 1,
      sym_string,
    STATE(100), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3320] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(101), 1,
      sym_comment,
    STATE(125), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3343] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(102), 1,
      sym_comment,
    STATE(122), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3366] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(284), 1,
      sym_identifier,
    ACTIONS(286), 1,
      anon_sym_RBRACE,
    STATE(103), 1,
      sym_comment,
    STATE(104), 1,
      aux_sym_block_repeat1,
    STATE(123), 1,
      sym_attribute,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3389] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(288), 1,
      anon_sym_RBRACE,
    ACTIONS(290), 1,
      sym_identifier,
    STATE(123), 1,
      sym_attribute,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    STATE(104), 2,
      sym_comment,
      aux_sym_block_repeat1,
  [3410] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(105), 1,
      sym_comment,
    STATE(146), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3433] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(106), 1,
      sym_comment,
    STATE(107), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3456] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(107), 1,
      sym_comment,
    STATE(126), 1,
      sym_string,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3479] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(293), 1,
      anon_sym_COMMA,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(244), 2,
      anon_sym_RBRACK,
      anon_sym_RPAREN,
    STATE(108), 2,
      sym_comment,
      aux_sym_array_repeat1,
  [3498] = 7,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(25), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 1,
      anon_sym_SQUOTE,
    ACTIONS(29), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(98), 1,
      sym_string,
    STATE(109), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3521] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(53), 1,
      anon_sym_RBRACK,
    ACTIONS(296), 1,
      anon_sym_COMMA,
    STATE(108), 1,
      aux_sym_array_repeat1,
    STATE(110), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3541] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(254), 1,
      anon_sym_RBRACE,
    ACTIONS(298), 1,
      anon_sym_COMMA,
    STATE(111), 1,
      sym_comment,
    STATE(115), 1,
      aux_sym_object_repeat1,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3561] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(57), 1,
      anon_sym_RBRACK,
    ACTIONS(300), 1,
      anon_sym_COMMA,
    STATE(108), 1,
      aux_sym_array_repeat1,
    STATE(112), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3581] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(250), 1,
      anon_sym_COMMA,
    ACTIONS(302), 1,
      anon_sym_RPAREN,
    STATE(108), 1,
      aux_sym_array_repeat1,
    STATE(113), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3601] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(304), 1,
      anon_sym_RBRACE,
    ACTIONS(306), 1,
      anon_sym_COMMA,
    STATE(114), 1,
      sym_comment,
    STATE(118), 1,
      aux_sym_object_repeat1,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3621] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(308), 1,
      anon_sym_RBRACE,
    ACTIONS(310), 1,
      anon_sym_COMMA,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    STATE(115), 2,
      sym_comment,
      aux_sym_object_repeat1,
  [3639] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(313), 1,
      anon_sym_RBRACE,
    ACTIONS(315), 1,
      anon_sym_COMMA,
    STATE(111), 1,
      aux_sym_object_repeat1,
    STATE(116), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3659] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(250), 1,
      anon_sym_COMMA,
    ACTIONS(317), 1,
      anon_sym_RPAREN,
    STATE(108), 1,
      aux_sym_array_repeat1,
    STATE(117), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3679] = 6,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(266), 1,
      anon_sym_RBRACE,
    ACTIONS(319), 1,
      anon_sym_COMMA,
    STATE(115), 1,
      aux_sym_object_repeat1,
    STATE(118), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3699] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(321), 1,
      sym_identifier,
    STATE(68), 1,
      sym_index_access,
    STATE(119), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3716] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(120), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(308), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [3731] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(323), 1,
      anon_sym_LBRACE,
    STATE(58), 1,
      sym_block,
    STATE(121), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3748] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(323), 1,
      anon_sym_LBRACE,
    STATE(53), 1,
      sym_block,
    STATE(122), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3765] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    STATE(123), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
    ACTIONS(325), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [3780] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(323), 1,
      anon_sym_LBRACE,
    STATE(52), 1,
      sym_block,
    STATE(124), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3797] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(323), 1,
      anon_sym_LBRACE,
    STATE(49), 1,
      sym_block,
    STATE(125), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3814] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(323), 1,
      anon_sym_LBRACE,
    STATE(57), 1,
      sym_block,
    STATE(126), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3831] = 5,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(327), 1,
      sym_identifier,
    STATE(27), 1,
      sym_index_access,
    STATE(127), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3848] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(258), 1,
      anon_sym_RBRACK,
    STATE(128), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3862] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(329), 1,
      anon_sym_SQUOTE,
    STATE(129), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3876] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(331), 1,
      anon_sym_SQUOTE,
    STATE(130), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3890] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(331), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(131), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3904] = 4,
    ACTIONS(333), 1,
      aux_sym_string_token2,
    ACTIONS(335), 1,
      anon_sym_POUND,
    STATE(132), 1,
      sym_comment,
    ACTIONS(337), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3918] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(339), 1,
      anon_sym_EQ,
    STATE(133), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3932] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(313), 1,
      anon_sym_RBRACE,
    STATE(134), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3946] = 4,
    ACTIONS(335), 1,
      anon_sym_POUND,
    ACTIONS(341), 1,
      aux_sym_string_token1,
    STATE(135), 1,
      sym_comment,
    ACTIONS(337), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3960] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(264), 1,
      anon_sym_RBRACK,
    STATE(136), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3974] = 4,
    ACTIONS(335), 1,
      anon_sym_POUND,
    ACTIONS(343), 1,
      aux_sym_string_token3,
    STATE(137), 1,
      sym_comment,
    ACTIONS(337), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [3988] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(329), 1,
      anon_sym_DQUOTE_DQUOTE_DQUOTE,
    STATE(138), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4002] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(345), 1,
      anon_sym_SLASH,
    STATE(139), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4016] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(347), 1,
      anon_sym_COLON,
    STATE(140), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4030] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(331), 1,
      anon_sym_DQUOTE,
    STATE(141), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4044] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(349), 1,
      ts_builtin_sym_end,
    STATE(142), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4058] = 4,
    ACTIONS(335), 1,
      anon_sym_POUND,
    ACTIONS(351), 1,
      aux_sym_comment_token1,
    STATE(143), 1,
      sym_comment,
    ACTIONS(337), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4072] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(304), 1,
      anon_sym_RBRACE,
    STATE(144), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4086] = 4,
    ACTIONS(335), 1,
      anon_sym_POUND,
    ACTIONS(353), 1,
      aux_sym_comment_token2,
    STATE(145), 1,
      sym_comment,
    ACTIONS(337), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4100] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(355), 1,
      anon_sym_EQ,
    STATE(146), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4114] = 4,
    ACTIONS(335), 1,
      anon_sym_POUND,
    ACTIONS(357), 1,
      aux_sym_string_token3,
    STATE(147), 1,
      sym_comment,
    ACTIONS(337), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4128] = 4,
    ACTIONS(3), 1,
      anon_sym_POUND,
    ACTIONS(329), 1,
      anon_sym_DQUOTE,
    STATE(148), 1,
      sym_comment,
    ACTIONS(5), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4142] = 4,
    ACTIONS(335), 1,
      anon_sym_POUND,
    ACTIONS(359), 1,
      aux_sym_string_token1,
    STATE(149), 1,
      sym_comment,
    ACTIONS(337), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4156] = 4,
    ACTIONS(335), 1,
      anon_sym_POUND,
    ACTIONS(361), 1,
      aux_sym_string_token2,
    STATE(150), 1,
      sym_comment,
    ACTIONS(337), 2,
      anon_sym_SLASH_SLASH,
      anon_sym_SLASH_STAR,
  [4170] = 1,
    ACTIONS(363), 1,
      ts_builtin_sym_end,
  [4174] = 1,
    ACTIONS(365), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 65,
  [SMALL_STATE(4)] = 130,
  [SMALL_STATE(5)] = 192,
  [SMALL_STATE(6)] = 254,
  [SMALL_STATE(7)] = 316,
  [SMALL_STATE(8)] = 378,
  [SMALL_STATE(9)] = 440,
  [SMALL_STATE(10)] = 502,
  [SMALL_STATE(11)] = 561,
  [SMALL_STATE(12)] = 620,
  [SMALL_STATE(13)] = 679,
  [SMALL_STATE(14)] = 738,
  [SMALL_STATE(15)] = 797,
  [SMALL_STATE(16)] = 856,
  [SMALL_STATE(17)] = 915,
  [SMALL_STATE(18)] = 952,
  [SMALL_STATE(19)] = 1011,
  [SMALL_STATE(20)] = 1070,
  [SMALL_STATE(21)] = 1129,
  [SMALL_STATE(22)] = 1172,
  [SMALL_STATE(23)] = 1207,
  [SMALL_STATE(24)] = 1242,
  [SMALL_STATE(25)] = 1279,
  [SMALL_STATE(26)] = 1316,
  [SMALL_STATE(27)] = 1348,
  [SMALL_STATE(28)] = 1380,
  [SMALL_STATE(29)] = 1430,
  [SMALL_STATE(30)] = 1478,
  [SMALL_STATE(31)] = 1509,
  [SMALL_STATE(32)] = 1540,
  [SMALL_STATE(33)] = 1571,
  [SMALL_STATE(34)] = 1602,
  [SMALL_STATE(35)] = 1633,
  [SMALL_STATE(36)] = 1664,
  [SMALL_STATE(37)] = 1695,
  [SMALL_STATE(38)] = 1726,
  [SMALL_STATE(39)] = 1757,
  [SMALL_STATE(40)] = 1788,
  [SMALL_STATE(41)] = 1819,
  [SMALL_STATE(42)] = 1850,
  [SMALL_STATE(43)] = 1881,
  [SMALL_STATE(44)] = 1914,
  [SMALL_STATE(45)] = 1945,
  [SMALL_STATE(46)] = 1976,
  [SMALL_STATE(47)] = 2007,
  [SMALL_STATE(48)] = 2040,
  [SMALL_STATE(49)] = 2061,
  [SMALL_STATE(50)] = 2082,
  [SMALL_STATE(51)] = 2103,
  [SMALL_STATE(52)] = 2124,
  [SMALL_STATE(53)] = 2145,
  [SMALL_STATE(54)] = 2166,
  [SMALL_STATE(55)] = 2187,
  [SMALL_STATE(56)] = 2214,
  [SMALL_STATE(57)] = 2241,
  [SMALL_STATE(58)] = 2262,
  [SMALL_STATE(59)] = 2283,
  [SMALL_STATE(60)] = 2318,
  [SMALL_STATE(61)] = 2343,
  [SMALL_STATE(62)] = 2368,
  [SMALL_STATE(63)] = 2403,
  [SMALL_STATE(64)] = 2424,
  [SMALL_STATE(65)] = 2450,
  [SMALL_STATE(66)] = 2482,
  [SMALL_STATE(67)] = 2514,
  [SMALL_STATE(68)] = 2544,
  [SMALL_STATE(69)] = 2566,
  [SMALL_STATE(70)] = 2598,
  [SMALL_STATE(71)] = 2628,
  [SMALL_STATE(72)] = 2658,
  [SMALL_STATE(73)] = 2680,
  [SMALL_STATE(74)] = 2710,
  [SMALL_STATE(75)] = 2742,
  [SMALL_STATE(76)] = 2763,
  [SMALL_STATE(77)] = 2784,
  [SMALL_STATE(78)] = 2809,
  [SMALL_STATE(79)] = 2830,
  [SMALL_STATE(80)] = 2859,
  [SMALL_STATE(81)] = 2884,
  [SMALL_STATE(82)] = 2905,
  [SMALL_STATE(83)] = 2926,
  [SMALL_STATE(84)] = 2949,
  [SMALL_STATE(85)] = 2970,
  [SMALL_STATE(86)] = 2991,
  [SMALL_STATE(87)] = 3012,
  [SMALL_STATE(88)] = 3033,
  [SMALL_STATE(89)] = 3054,
  [SMALL_STATE(90)] = 3075,
  [SMALL_STATE(91)] = 3096,
  [SMALL_STATE(92)] = 3117,
  [SMALL_STATE(93)] = 3138,
  [SMALL_STATE(94)] = 3159,
  [SMALL_STATE(95)] = 3180,
  [SMALL_STATE(96)] = 3204,
  [SMALL_STATE(97)] = 3228,
  [SMALL_STATE(98)] = 3251,
  [SMALL_STATE(99)] = 3274,
  [SMALL_STATE(100)] = 3297,
  [SMALL_STATE(101)] = 3320,
  [SMALL_STATE(102)] = 3343,
  [SMALL_STATE(103)] = 3366,
  [SMALL_STATE(104)] = 3389,
  [SMALL_STATE(105)] = 3410,
  [SMALL_STATE(106)] = 3433,
  [SMALL_STATE(107)] = 3456,
  [SMALL_STATE(108)] = 3479,
  [SMALL_STATE(109)] = 3498,
  [SMALL_STATE(110)] = 3521,
  [SMALL_STATE(111)] = 3541,
  [SMALL_STATE(112)] = 3561,
  [SMALL_STATE(113)] = 3581,
  [SMALL_STATE(114)] = 3601,
  [SMALL_STATE(115)] = 3621,
  [SMALL_STATE(116)] = 3639,
  [SMALL_STATE(117)] = 3659,
  [SMALL_STATE(118)] = 3679,
  [SMALL_STATE(119)] = 3699,
  [SMALL_STATE(120)] = 3716,
  [SMALL_STATE(121)] = 3731,
  [SMALL_STATE(122)] = 3748,
  [SMALL_STATE(123)] = 3765,
  [SMALL_STATE(124)] = 3780,
  [SMALL_STATE(125)] = 3797,
  [SMALL_STATE(126)] = 3814,
  [SMALL_STATE(127)] = 3831,
  [SMALL_STATE(128)] = 3848,
  [SMALL_STATE(129)] = 3862,
  [SMALL_STATE(130)] = 3876,
  [SMALL_STATE(131)] = 3890,
  [SMALL_STATE(132)] = 3904,
  [SMALL_STATE(133)] = 3918,
  [SMALL_STATE(134)] = 3932,
  [SMALL_STATE(135)] = 3946,
  [SMALL_STATE(136)] = 3960,
  [SMALL_STATE(137)] = 3974,
  [SMALL_STATE(138)] = 3988,
  [SMALL_STATE(139)] = 4002,
  [SMALL_STATE(140)] = 4016,
  [SMALL_STATE(141)] = 4030,
  [SMALL_STATE(142)] = 4044,
  [SMALL_STATE(143)] = 4058,
  [SMALL_STATE(144)] = 4072,
  [SMALL_STATE(145)] = 4086,
  [SMALL_STATE(146)] = 4100,
  [SMALL_STATE(147)] = 4114,
  [SMALL_STATE(148)] = 4128,
  [SMALL_STATE(149)] = 4142,
  [SMALL_STATE(150)] = 4156,
  [SMALL_STATE(151)] = 4170,
  [SMALL_STATE(152)] = 4174,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_runbook, 0),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(135),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(34),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(32),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [45] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [65] = {.entry = {.count = 1, .reusable = false}}, SHIFT(149),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [73] = {.entry = {.count = 1, .reusable = false}}, SHIFT(89),
  [75] = {.entry = {.count = 1, .reusable = false}}, SHIFT(94),
  [77] = {.entry = {.count = 1, .reusable = false}}, SHIFT(93),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [81] = {.entry = {.count = 1, .reusable = false}}, SHIFT(47),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_reference, 1),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [93] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [95] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_reference, 1),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_reference_repeat1, 2),
  [99] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_reference_repeat1, 2), SHIFT_REPEAT(127),
  [102] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_reference_repeat1, 2),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_reference, 2),
  [106] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_reference, 2),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_index_access, 4),
  [110] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_index_access, 4),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_runbook, 1),
  [114] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_runbook_repeat1, 2),
  [116] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_runbook_repeat1, 2), SHIFT_REPEAT(101),
  [119] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_runbook_repeat1, 2), SHIFT_REPEAT(106),
  [122] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_runbook_repeat1, 2), SHIFT_REPEAT(109),
  [125] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_runbook_repeat1, 2), SHIFT_REPEAT(99),
  [128] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_runbook_repeat1, 2), SHIFT_REPEAT(102),
  [131] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_runbook_repeat1, 2), SHIFT_REPEAT(105),
  [134] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_runbook_repeat1, 2), SHIFT_REPEAT(100),
  [137] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array, 3),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array, 3),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array, 2),
  [143] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array, 2),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1),
  [151] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1),
  [155] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 1),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, .production_id = 7),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, .production_id = 7),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object, 3),
  [163] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object, 3),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object, 2),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object, 2),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, .production_id = 9),
  [171] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, .production_id = 9),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array, 5),
  [175] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array, 5),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object, 5),
  [179] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object, 5),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_expression, 3),
  [183] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_expression, 3),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, .production_id = 8),
  [187] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, .production_id = 8),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [191] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array, 4),
  [195] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array, 4),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object, 4),
  [199] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object, 4),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_input_declaration, 4, .production_id = 5),
  [203] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [205] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [207] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [209] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_runbook_repeat1, 1),
  [213] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_addon_block, 3, .production_id = 2),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 2, .production_id = 1),
  [217] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__statement, 1),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_output_block, 3, .production_id = 3),
  [221] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_declaration, 3, .production_id = 3),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [225] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_signer_block, 4, .production_id = 4),
  [227] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_block, 4, .production_id = 4),
  [229] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [233] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [235] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_reference_repeat1, 2), SHIFT_REPEAT(119),
  [238] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [240] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [242] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [244] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_repeat1, 2),
  [246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [254] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [256] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [264] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [266] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [268] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 3, .production_id = 6),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [272] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [276] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_field, 3, .production_id = 6),
  [278] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [280] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [288] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2),
  [290] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(133),
  [293] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_repeat1, 2), SHIFT_REPEAT(15),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [298] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [300] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [302] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [304] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [306] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [308] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_object_repeat1, 2),
  [310] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_object_repeat1, 2), SHIFT_REPEAT(79),
  [313] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [315] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [317] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [319] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [321] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [323] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [325] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 1),
  [327] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [329] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [331] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [333] = {.entry = {.count = 1, .reusable = false}}, SHIFT(129),
  [335] = {.entry = {.count = 1, .reusable = false}}, SHIFT(143),
  [337] = {.entry = {.count = 1, .reusable = false}}, SHIFT(145),
  [339] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [341] = {.entry = {.count = 1, .reusable = false}}, SHIFT(148),
  [343] = {.entry = {.count = 1, .reusable = false}}, SHIFT(138),
  [345] = {.entry = {.count = 1, .reusable = false}}, SHIFT(151),
  [347] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [349] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [351] = {.entry = {.count = 1, .reusable = false}}, SHIFT(152),
  [353] = {.entry = {.count = 1, .reusable = false}}, SHIFT(139),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [357] = {.entry = {.count = 1, .reusable = false}}, SHIFT(131),
  [359] = {.entry = {.count = 1, .reusable = false}}, SHIFT(141),
  [361] = {.entry = {.count = 1, .reusable = false}}, SHIFT(130),
  [363] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 3),
  [365] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 2),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_txtx(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
