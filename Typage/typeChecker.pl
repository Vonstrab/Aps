/*
Main
*/

main_stdin :-
    read(user_input,T),
    typeProg(T,R),
    print(R),
    nl,
    halt.

/*
Traduction
*/

typeProg(cdms(D,CS) , void) :- typeProg(typeCMDS([], D,CS, void ),void).
typeProg(stat(X) , void) :- typeProg(typeStat([], X , void ),void).


typeCMDS(CTX , stat(S) , cdms(E1 ,E2) , void ) :- 
    typeCMDS(CTX , typeStat(CTX , S ,void ) , typeCMDS(CTX ,E1, E2 , void) , void ).

typeCMDS(CTX , dec(D) , cdms(E1 ,E2) , void ) :- 
    typeCMDS(CTX , typeDec(CTX ,D,CTX2 ) , typeCMDS(CTX2  ,E1, E2 , void) , void ).

typeCMDS(CTX , dec(D) , stat(S2) , void ) :- 
    typeCMDS(CTX , typeDec(CTX , D ,CTX2 ) , typeStat(CTX2  ,S2 , void) , void ).

/*
typeExpr
*/

typeExpr(_,integer(X),int):-typeExpr(_,X,int).
typeExpr(_,false,bool). 
typeExpr(_,true,bool).

typeExpr(CTX, id(X), T):-dansGram(CTX,X,T).

typeExpr(CTX, exprIf(E1 ,E2,E3 ), T) :- 
    typeExpr(CTX, E1 , bool),
    typeExpr(CTX, E2 , T),
    typeExpr(CTX, E3 , T).

typeExpr(CTX,unOp(X),T):-typeExpr(CTX, X ,T).

typeExpr(CTX,binOpBool(X,Y),bool ):- typeExpr(CTX, X ,bool),typeExpr(CTX, Y ,bool).    
typeExpr(CTX,binOpIntBool(X,Y),bool ):- typeExpr(CTX, X ,int),typeExpr(CTX, Y ,int).    
typeExpr(CTX,binOpInt(X,Y),int ):- typeExpr(CTX, X ,int),typeExpr(CTX, Y ,int).    

typeExpr(CTX,app(X,Y),Type):- 
    dansGram(CTX, X ,T),
    nth(0,T,Type),
    remove(T),
    lenght(T,LT),
    lenght(Y,LT),
    typeExpr(CTX, Y ,Type).

typeExpr(CTX,app( abs(X,Z) , Y ),Type):-
    append(CTX,X ,CTX2),
    typeExpr(CTX2, Z ,Type),
    typeExpr(CTX2, Y ,Type).

tymeExpr(CTX,len(E),int):-dansGram(CTX,E,vec(_)).
typeExpr(CTX,alloc(E),vec(_)):-typeExpr(CTX,E,int).
typeExpr(CTX,expnth(L,E),T):-
    typeExpr(CTX,L,vec(T)),
    typeExpr(CTX,E,int).

typeExpr(CTX,abs(Args,E),T):-
    append(CTX,Args ,CTXfunc),
    typefunc(Typef, Args , T),
    typeExpr(CTXfunc , E , T).
    

/*
Manipulations de gramaire
*/

dansGram([(Var,Type)|_] ,Var,Type).
dansGram([(_,_) |Tail] ,X,T) :- dansGram(Tail ,X,T).

remove([_|Tail]) :- remove(Tail), !.  

typefunc(Type,[],T) :- append(T,Type,Type).
typefunc(Type,[(_,T)| Tail],T) :- append( Type, T ,Type1), typefunc(Type1,Tail,T,_).


/*
Declarations
*/

typeDec(CTX,const(X,T,E),[(X , T)| CTX]) :- typeExpr(CTX , E , T).

typeDec(CTX,var(X,T),[(X , T)| CTX]).

typeDec(CTX,proc(X,Args,T,E),[(X , Typef)| CTX]) :- 
    append(CTX,Args ,CTXfunc),
    typefunc(Typef, Args , void),
    typeExpr(CTXfunc , E , T).
    
typeDec(CTX,procRec(X,Args,T,E),[(X , Typef)| CTX]  ) :- 
    append(CTX,Args ,CTXfunc),
    typefunc(Typef, Args,void),
    typeExpr([typef|CTXfunc] , E,T).

typeDec(CTX,fonction(X,Args,T,E),[(X , Typef)| CTX]) :- 
    append(CTX,Args ,CTXfunc),
    typefunc(Typef, Args , T),
    typeExpr(CTXfunc , E , T).
    
typeDec(CTX,fonctionRec(X,Args,T,E),[(X , Typef)| CTX]  ) :- 
    append(CTX,Args ,CTXfunc),
    typefunc(Typef, Args,T),
    typeExpr([typef|CTXfunc] , E,T).


/*
Stat
*/

typeStat(CTX, echo(S),void):- typeExpr(CTX,S,int).

typeStat(CTX, statIf(Cond,Else,Then),void):- 
    typeExpr(CTX,Cond,bool),
    typeProg(Else,void),
    typeProg(Then,void).


typeStat(CTX, swhile(Cond,Loop),void):- 
    typeExpr(CTX,Cond,bool),
    typeProg(Loop,void).

typeStat(CTX, call(Proc,Args),void):- 
    dansGram(CTX,Proc,T),
    nth(0,T,void),
    remove(T),
    lenght(T,LT),
    lenght(Args,LT),
    check(CTX,T,Args).

check(CTX,[H|T],[Arg|Args]):-
    typeExpr(CTX, Arg ,H),
    check(CTX,T,Args).

typeStat(CTX, set(L,Expr),void):-
    dansGram(CTX,L,Type),
    typeExpr(CTX,Expr,Type).


/*
CDMS
*/

typeCMDS(CTX , typeDec(CTX, _ ,CTX2) , typeCMDS(CTX2  ,_, _ , void) , void ) .
typeCMDS(CTX , typeStat(CTX , _ , void) , typeCMDS(CTX,_,_ ,void) ,void ) .
typeCMDS(CTX , typeDec(CTX, _ ,CTX2) , typeStat(CTX2,_ , void) , void ) .

/*
prog
*/
typeProg(typeCMDS([], D,CS, void ),void) :- typeCMDS([], D,CS, void ).
typeProg(typeStat([], S, void ),void) :- typeStat([], S, void ).