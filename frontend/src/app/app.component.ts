import { Component } from '@angular/core';
import { MenuType } from './main/dto/menu-type';
import { ListType } from './main/dto/list-type';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'chess';
  menu: MenuType = "chat";
  listType: ListType = "none";
  registerCard: boolean = false;

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
}
