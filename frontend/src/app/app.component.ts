import { Component } from '@angular/core';
import { MenuType } from './main/dto/menu-type';
import { ListType } from './main/dto/list-type';
import { GameType } from './main/dto/game-type';
import { GameRequest } from './main/dto/game-request';
import { GameManagementService } from './main/game-management.service';
import { GameResponse } from './main/dto/game-response';
import { HttpErrorResponse } from '@angular/common/http';
import { ToastService } from './elements/toast.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'chess';
  menu: MenuType = "chat";
  aiGame: boolean = false;
  listType: ListType = "none";
  registerCard: boolean = false;

  constructor(private service: GameManagementService, private toast: ToastService) {}

  openChat() {
    this.menu = "chat";
  }

  openList(listType: ListType) {
    this.menu = "list";
    this.listType = listType;
  }

  get logged(): boolean {
    return localStorage.getItem("username") != null;
  }

  changeRegisterCard(value: boolean) {
    this.registerCard = value;
  }

  createGame(event: GameType) {
    this.aiGame = event == "AI";
    this.menu = "new";
  }

  doCreateGame(request: GameRequest) {
    this.service.newGame(request).subscribe({
      next: (response: GameResponse) => this.onGameCreation(response),
      error: (error: HttpErrorResponse) => this.onError(error),
    });
  }

  onGameCreation(response: GameResponse) {
    // TODO
  }

  onError(error: HttpErrorResponse) {
    this.toast.createToast({message: error.error.message, id: `error-${new Date().toTimeString}`, type: "error"});
  }

}
