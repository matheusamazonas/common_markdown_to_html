# Common Markdown to HTML

A Markdown (CommonMark) parser and HTML converter written in Rust.

⚠️ This project is still under development and is not recommended for production use.

## Goal
To write a Markdown parser that is CommonMark-compliant, along with an HTML converter.

## Spec coverage
| Section                                 | Progress          | Spec tests |
|-----------------------------------------|-------------------|-----------:|
| Tabs                                    | Not implemented ❌ |       2/11 |
| Backslash escapes                       | Not implemented ❌ |       0/13 |
| Entity and numeric character references | Not implemented ❌ |       1/17 |
| Precedence                              | Not implemented ❌ |        0/1 |
| Thematic breaks                         | Implemented ✅     |      13/19 |
| ATX headings                            | Implemented ✅     |      13/18 |
| Setext headings                         | Not implemented ❌ |       4/27 |
| Indented code blocks                    | Not implemented ❌ |       1/12 |
| Fenced code blocks                      | Not implemented ❌ |       0/29 |
| HTML blocks                             | Not implemented ❌ |       1/44 |
| Link reference definitions              | Not implemented ❌ |       4/27 |
| Paragraphs                              | Implemented ✅     |        7/8 |
| Blank lines                             | Not implemented ❌ |        0/1 |
| Block quotes                            | Not implemented ❌ |       0/25 |
| List items                              | Not implemented ❌ |       5/48 |
| Lists                                   | Not implemented ❌ |       6/26 |
| Code spans                              | Not implemented ❌ |       3/22 |
| Emphasis and strong emphasis            | Not implemented ❌ |     38/132 |
| Links                                   | Not implemented ❌ |      16/90 |
| Images                                  | Not implemented ❌ |       0/22 |
| Autolinks                               | Not implemented ❌ |       2/19 |
| Raw HTML                                | Not implemented ❌ |      12/20 |
| Hard line breaks                        | Not implemented ❌ |       8/15 |
| Soft line breaks                        | Not implemented ❌ |      2/2 ✅ |
| Textual content                         | Not implemented ❌ |      3/3 ✅ |
| **Total**                               |                   |    132/652 | 

## Dependencies
Common Markdown to HTML has the following dependencies:
- [`yapcol`](https://github.com/matheusamazonas/yapcol): parser combinator library.

## License
Common Markdown to HTML is distributed under the terms of the MIT license. For more information, check the [LICENSE](LICENSE.md) file in
this repository.