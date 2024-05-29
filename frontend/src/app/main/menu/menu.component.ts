import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { ListType } from '../dto/list-type';
import { GameType } from '../dto/game-type';

@Component({
  selector: 'app-menu',
  templateUrl: './menu.component.html',
  styleUrls: ['./menu.component.css']
})
export class MenuComponent implements OnInit {
  requestView: boolean = false;
  activeView: boolean = false;
  error: boolean = false;
  errorMsg: String = "";

  @Output() openGame: EventEmitter<number> = new EventEmitter<number>();
  @Output() openGameModal: EventEmitter<GameType> = new EventEmitter<GameType>();
  @Output("get") getList: EventEmitter<ListType> = new EventEmitter<ListType>();

  constructor() { }

  ngOnInit(): void {
  }

  getRequests() {
    this.getList.emit("requests");
  }

  getGames() {
    this.getList.emit("games");
  }

  getArchive() {
    this.getList.emit("archive");
  }

  newGame() {
    this.openGameModal.emit("User");
  }

  newAIGame() {
    this.openGameModal.emit("AI");
  }

  open(gameId: number) {
    this.openGame.emit(gameId);
  }

  logout() {
    localStorage.removeItem('refresh');
    localStorage.removeItem('token');
    localStorage.removeItem('username');
  }
}
