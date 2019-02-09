import { LitElement, html, customElement, property } from 'lit-element';
import { connect } from 'pwa-helpers/connect-mixin.js';
import { store } from '../../store';
import { createNote } from '../state/actions';
import {
  createContext,
  createCommit,
  createContextAndCommitFromObject
} from '../../vc/state/actions';

@customElement('create-note')
export class CreateNote extends connect(store)(LitElement) {
  render() {
    return html`
      <div>
        <input />
        <textarea></textarea>
        <button @click="${this.createNote}">Create note</button>
      </div>
    `;
  }

  createNote() {
    store
      .dispatch(
        createNote.create({ title: 'cardtitle', content: 'cardcontent' })
      )
      .then(cardAddress => {
        createContextAndCommitFromObject(
          store,
          'contextName',
          'message',
          cardAddress
        );
      });
  }
}