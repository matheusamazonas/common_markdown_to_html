# Common Markdown to HTML

A Markdown (CommonMark) parser and HTML converter written in Rust.

⚠️ This project is still under development and is not recommended for production use.

## Goal
To write a Markdown parser that is CommonMark-compliant, along with an HTML converter.

## Spec coverage
| Section                                 | Progress             | Examples | Tests |
|-----------------------------------------|----------------------|---------:|-------|
| Tabs                                    | Not implemented ❌    |     0/11 |       |
| Backslash escapes                       | Not implemented ❌    |     0/13 |       |
| Entity and numeric character references | Not implemented ❌    |     0/17 |       |
| Precedence                              | Not implemented ❌    |      0/1 |       |
| Thematic breaks                         | Implemented ✅        | 13/19 ⚠️ |       |
| ATX headings                            | Under development ⚠️ |     0/18 |       |
| Setext headings                         | Not implemented ❌    |     0/27 |       |
| Indented code blocks                    | Not implemented ❌    |     0/12 |       |
| Fenced code blocks                      | Not implemented ❌    |     0/29 |       |
| HTML blocks                             | Not implemented ❌    |     0/44 |       |
| Link reference definitions              | Not implemented ❌    |     0/27 |       |
| Paragraphs                              | Not implemented ❌    |      0/8 |       |
| Blank lines                             | Not implemented ❌    |      0/1 |       |
| Block quotes                            | Not implemented ❌    |     0/25 |       |
| List items                              | Not implemented ❌    |     0/48 |       |
| Lists                                   | Not implemented ❌    |     0/26 |       |
| Code spans                              | Not implemented ❌    |     0/22 |       |
| Emphasis and strong emphasis            | Not implemented ❌    |    0/132 |       |
| Links                                   | Not implemented ❌    |     0/90 |       |
| Images                                  | Not implemented ❌    |     0/22 |       |
| Autolinks                               | Not implemented ❌    |     0/19 |       |
| Raw HTML                                | Not implemented ❌    |     0/20 |       |
| Hard line breaks                        | Not implemented ❌    |     0/15 |       |
| Soft line breaks                        | Not implemented ❌    |      0/2 |       |
| Textual content                         | Not implemented ❌    |      0/3 |       |

## Dependencies
Common Markdown to HTML has the following dependencies:
- [`yapcol`](https://github.com/matheusamazonas/yapcol): parser combinator library.

## License
Common Markdown to HTML is distributed under the terms of the MIT license. For more information, check the [LICENSE](LICENSE.md) file in
this repository.