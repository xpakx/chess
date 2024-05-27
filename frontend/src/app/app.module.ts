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

@NgModule({
  declarations: [
    AppComponent,
    ModalLoginComponent,
    ModalRegisterComponent,
    BoardComponent,
    ChatComponent,
    MenuComponent,
    ToastComponent
  ],
  imports: [
    BrowserModule,
    HttpClientModule, 
    FormsModule,
    ReactiveFormsModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
