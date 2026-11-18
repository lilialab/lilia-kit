import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { greet as greetFromRust } from './lib/tauri/greet';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
})
export class AppComponent {
  greetingMessage = signal('');

  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    void greetFromRust(name).then((text) => {
      this.greetingMessage.set(text);
    });
  }
}
