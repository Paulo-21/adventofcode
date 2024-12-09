:- set_prolog_flag(double_quotes, codes).
:- use_module(library(dcg/basics)).
:- use_module(library(pio)).

% edit your input path and run: swipl -t go c:/path/to/2024-day07.pl
inputPath('C:/Users/paulc/Desktop/code/adventofcode/2024/day7/input').

:- dynamic op_concat/3.
op_concat(Acc, N, Acc2) :- fail.   % code is swapped in for Part 2.

% try to parse line as valid equation, fallback read as 0 and skip over.
eq(Test) --> number(Test), ": ", number(N1), eq_chain(Test, N1).
eq(0) --> string(_), eol.

% chain of spaces and numbers, trying each op in turn on backtracking.
eq_chain(Test, Acc) --> " ", number(N),
    {(    X is Acc + N
     ;    X is Acc * N
     ;    op_concat(Acc, N, X)
     ), X =< Test
    }, eq_chain(Test, X).

% end of chain, test and calc match, or fail, backtrack, try other ops.
eq_chain(T,T) --> eol.

% a list of equations is zero or more equations.
eqs([]) --> [].
eqs([T|Ts]) --> eq(T), eqs(Ts).

go :-
    inputPath(P),                    % filename.
    phrase_from_file(eqs(Eqs), P),   % parse input.
    sum_list(Eqs, P1Sum),
    format('Part 1 Sum: ~w~n', [P1Sum]),

    % Part 2 - remove part 1 check, define check for part 2.
    retractall(op_concat(_,_,_)),
    % new op is: "how wide is the number on the right in base 10 columns?
    %             multiply left by 10^<that> to make room".
    assertz((op_concat(Acc, N, X) :- X is (Acc*10^floor(log10(N)+1)) + N)),

    phrase_from_file(eqs(Eqs2), P),   % parse input.
    sum_list(Eqs2, P2Sum),
    format('Part 2 Sum: ~w~n', [P2Sum]).