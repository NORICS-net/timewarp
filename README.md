# timewarp
NLP library for parsing English and German natural language into dates and times. 
Leverages [pest](https://pest.rs) for parsing human readable-dates and times.

## Examples:

### Direct Input

To input a `2022-12-01` you can type:  `12/1/22`, `01.12.22`, `22-12-01`.

### Week

`2022-W52`, `2022W52`, `week 22-52` or `KW 22/52` are interpreted as an 
intervall `2022-12-26 <= x < 2023-01-02`. 

### Relative Dates

`yesterday`, `tomorrow`, etc. are calculated based of a given base.
`+4 weeks`, `-5 months` 

