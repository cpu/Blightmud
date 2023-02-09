# Spellcheck

This module offers methods to perform spellchecking using a Hunspell 
compatible dictionary. Before use, the spellcheck module must be
initialized by providing paths to an AFF file, and a dictionary file.

You may find compatible dictionaries here:
  https://github.com/wooorm/dictionaries 

##

***spellcheck.init(aff_path, dict_path)***
Initializes spellchecking using the provided paths.

- `aff_path`    path to a Hunspell affix file.
- `dict_path`   path to a Hunspell dictionary file.

##

***spellcheck.check(word) -> bool***
Checks whether the given word exists in the dictionary. If called
before `spellcheck.init` an error will be produced.

- `word`    A potentially misspelled word.

##

***spellcheck.suggest(word) -> table***
Returns a table of suggested replacements for a misspelled word. 
If called before `spellcheck.init` an error will be produced.

- `word`    A potentially misspelled word.
- `table`   An array consisting of candidate replacement words, in
            order of likelihood.

