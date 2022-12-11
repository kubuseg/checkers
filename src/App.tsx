import React, { useEffect, useState } from 'react';
import './App.css';
import init, { add } from "rust-wasm-lib";

function Square(props: { onClick: React.MouseEventHandler<HTMLButtonElement>; value: number; }) {
  return (
    <button className="square" onClick={props.onClick}>
      {props.value}
    </button>
  );
}

class Board extends React.Component<any, any> {
  constructor(props : any) {
    super(props);
    this.state = {
      squares: Array(9).fill(null),
      xIsNext: true,
    };
  }

  handleClick(i : number) {
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
    return (
      <Square
        value={this.state.squares[i]}
        onClick={() => this.handleClick(i)}
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

    const rows = [];
    for (let i = 0; i < 36; i+=6) {
      rows.push(
        <div className="board-row">
          {this.renderSquare(i)}
          {this.renderSquare(i+1)}
          {this.renderSquare(i+2)}
          {this.renderSquare(i+3)}
          {this.renderSquare(i+4)}
          {this.renderSquare(i+5)}
        </div>
      );
    }
    return (
      <div>
        <div className="status">{status}</div>
        <tbody>{rows}</tbody>
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

function calculateWinner(squares: any[]) {
  return null;
}

