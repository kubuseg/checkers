import React, { useEffect, useState } from 'react';
import './App.css';
import init, { add } from "rust-wasm-lib";

function Square(props: { onClick: React.MouseEventHandler<HTMLButtonElement>; 
                         value: string; color:string; }) 
{
  return (
    <button className="square" style={{ backgroundColor: props.color}} onClick={props.onClick}>
      {props.value}
    </button>
  );
}

interface IBoardState {
  squares: string[];
  xIsNext: boolean;
}

class Board extends React.Component<{}, IBoardState> {
  constructor(props : IBoardState) {
    super(props);
    this.state = {
      squares: Array(100).fill(null),
      xIsNext: true,
    };
  }

  handleClick(i:number) {
    const squares = this.state.squares.slice();
    if (calculateWinner(squares) || squares[i]) {
      return;
    }
    squares[i] = this.state.xIsNext ? 'X' : 'O';
    this.setState({
      squares: squares,
      xIsNext: !this.state.xIsNext,
    });
  }

  
  renderSquare(i: number) {
    const backgroundColor = [1, 3, 5, 7, 9].includes(Math.abs(i%10-Math.floor(i/10))%11) ? "#693e3e" : "#ffd6ae";
    return (
      <Square
        value={this.state.squares[i]}
        onClick={() => this.handleClick(i)}
        color={backgroundColor}
      />
    );
  }

  render() {
    const winner = calculateWinner(this.state.squares);
    let status;
    if (winner) {
      status = 'Winner: ' + winner;
    } else {
      status = 'Next player: ' + (this.state.xIsNext ? 'X' : 'O');
    }

    const board = [];
    for(let i = 0; i < 10; i++){
      const squareRows = [];
      for(let j = 0; j < 10; j++){
        const backgroundColor = [1, 3, 5, 7, 9].includes(Math.abs(j-i)%11) ? "#693e3e" : "#ffd6ae";
        squareRows.push(
          <Square
          value={this.state.squares[i*10+j]}
          onClick={() => this.handleClick(i*10+j)}
          color={backgroundColor}
        />
        );
      }
      board.push(<div className="board-row">{squareRows}</div>)
    }

    return (
      <div>
        <div className="status">{status}</div>
        <div>
          {board}
        </div>
      </div>
    );
  }
}

class Game extends React.Component {
  render() {
    return (
      <div className="game">
        <div className="game-board">
          <Board />
        </div>
        <div className="game-info">
          <div>{/* status */}</div>
          <ol>{/* TODO */}</ol>
        </div>
      </div>
    );
  }
}

export default Game;

function calculateWinner(squares: string[]) {
  return null;
}

