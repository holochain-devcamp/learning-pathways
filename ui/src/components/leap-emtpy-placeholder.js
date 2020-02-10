import { LitElement, css, html } from 'lit-element';
import { sharedStyles } from '../shared-styles';

export class LeapEmtpyPlacholder extends LitElement {
  static get styles() {
    return [
      sharedStyles,
      css`
        .message {
          font-size: 18px;
          text-align: center;
        }

        .container {
          opacity: 0.6;
        }
        mwc-icon {
          --mdc-icon-size: 64px;
        }
        :host {
          display: flex;
          flex: 1;
        }
      `
    ];
  }

  static get properties() {
    return {
      message: {
        type: String
      }
    };
  }

  render() {
    return html`
      <div class="container column fill center-content">
        <mwc-icon>filter_drama</mwc-icon>
        <span class="message">${this.message || 'No items in this list'}</span>
      </div>
    `;
  }
}
