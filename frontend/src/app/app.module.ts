import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppComponent } from './app.component';
import { HttpClientModule } from "@angular/common/http";
import { ModalLoginComponent } from './auth/modal-login/modal-login.component';
import { ModalRegisterComponent } from './auth/modal-register/modal-register.component'
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { BoardComponent } from './board/board/board.component';
import { ChatComponent } from './chat/chat.component';
import { MenuComponent } from './main/menu/menu.component';
import { ToastComponent } from './elements/toast/toast.component';
import { FieldPipe } from './board/field.pipe';
import { GameListComponent } from './main/game-list/game-list.component';
import { NewGameModalComponent } from './main/new-game-modal/new-game-modal.component';
import { RefreshButtonComponent } from './elements/refresh-button/refresh-button.component';

@NgModule({
  declarations: [
    AppComponent,
    ModalLoginComponent,
    ModalRegisterComponent,
    BoardComponent,
    ChatComponent,
    MenuComponent,
    ToastComponent,
    FieldPipe,
    GameListComponent,
    NewGameModalComponent,
    RefreshButtonComponent
  ],
  imports: [
    BrowserModule,
    HttpClientModule, 
    FormsModule,
    ReactiveFormsModule,
  ],
  providers: [
    FieldPipe,
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
