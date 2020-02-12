import { LitElement, html, css } from 'lit-element';

import '@authentic/mwc-circular-progress';
import '@authentic/mwc-card';
import '@authentic/mwc-list';

import { sharedStyles } from '../shared-styles';
import { getClient } from '../graphql';
import { GET_VALID_MEMBERS } from '../graphql/queries';

export class LeapMembersList extends LitElement {
  static get properties() {
    return {
      members: {
        type: Array
      }
    };
  }

  async firstUpdated() {
    const client = await getClient();

    const result = await client.query({ query: GET_VALID_MEMBERS });
    this.members = result.data.validMembers;
  }

  static get styles() {
    return sharedStyles;
  }

  render() {
    return html`
      <mwc-card style="width: auto;">
        <div class="column center-content">
          ${this.members
            ? html`
                <mwc-list>
                  ${this.members.map(
                    member =>
                      html`
                        <mwc-list-item>${member}</mwc-list-item>
                      `
                  )}
                </mwc-list>
              `
            : html`
                <mwc-circular-progress
                  style="padding: 56px;"
                ></mwc-circular-progress>
              `}
        </div>
      </mwc-card>
    `;
  }
}
