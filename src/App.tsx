import React, { useEffect, useState } from 'react';
import './App.css';
import init, { add } from "rust-wasm-lib";

interface IFigure { 
  color: "black" | "white";
  type: "man" | "king";
}

function Figure(props: IFigure)
{
  return (
    <div className="circle" style={{ backgroundColor: props.color}}/>
  );
}

interface ISquareProps {
  onClick: React.MouseEventHandler<HTMLButtonElement>; 
  value: IFigure | null;
  color: string;
}

function Square(props: ISquareProps) 
{
  return (
    <button 
    className="square"
    style={{ backgroundColor: props.color}} 
    onClick={props.onClick}>
      {props.value && <Figure color={props.value.color} type={props.value.type} />}
    </button>
  );
}

interface IBoardProps {
  squareValue: (IFigure | null)[];
  selectedSquare: number | null;
  onClick: (squareNo:number, squareValue:IFigure|null) => void;
}

function Board(props: IBoardProps) {
  const board = [];
  for(let i = 0; i < 10; i++){
    const squareRows = [];
    for(let j = 0; j < 10; j++){
      const squareNo = i*10+j;
      const backgroundColor = 
      isOnDarkDiag(squareNo) ? "#693e3e" : "#ffd6ae";
      const squareValue = props.squareValue[squareNo];
      squareRows.push(
        <Square
        value={props.squareValue[squareNo]}
        onClick={() => props.onClick(squareNo, squareValue)}
        color={props.selectedSquare === squareNo ? 'sandybrown' : backgroundColor}
      />
      );
    }
    board.push(<div className="board-row">{squareRows}</div>)
  }

  return (
    <div>
      {board}
    </div>
  );
}

interface IGameState {
  squareValue: (IFigure | null)[];
  seletedSquare: number | null;
  whiteIsNext: boolean;
}

export default class Game extends React.Component<{}, IGameState> {
  constructor(props : IGameState) {
    super(props);
    this.state = {
      squareValue: Array(100).fill(null)
      .map((val:(IFigure | null), i:number) => {
          if (isOnDarkDiag(i)) {
            if (i < 30)
              return {color:'black', type:'man'};
            if (i >= 70)
              return {color:'white', type:'man'};
          }
          return null;
      }),
      seletedSquare: null,
      whiteIsNext: true,
    };

  }

  handleClick = (i:number, figure:IFigure|null) => {
    const squares = this.state.squareValue.slice();
    let selectedSquare = null;
    if (squares[i] !== null) {
      if (this.state.seletedSquare && this.state.seletedSquare === i)
        selectedSquare = null;
      else 
        selectedSquare = i;
    }

    this.setState({
      squareValue: squares,
      seletedSquare: selectedSquare,
    });
  }

  render() {
    const winner = calculateWinner(this.state.squareValue);
    let status;
    if (winner)
      status = 'Winner: ' + winner;
    else
      status = 'Next player: ' + (this.state.whiteIsNext ? 'White' : 'Black');
    return (
      <div className="game">
        <div className="game-board">
        <div className="status">{status}</div>
          <Board 
          squareValue={this.state.squareValue}
          selectedSquare={this.state.seletedSquare}
          onClick={this.handleClick}
          />
        </div>
      </div>
    );
  }
}

const isOnDarkDiag = (i:number) : boolean => {
  return [1, 3, 5, 7, 9].includes(Math.abs(i%10-Math.floor(i/10))%11)
}


function getPossibleMoves(square: any, figure: any){
}

function calculateWinner(squares: any[]){
  return null;
}

