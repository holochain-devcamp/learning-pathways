import { LitElement, html, css } from 'lit-element';

import '@authentic/mwc-card';
import '@authentic/mwc-list';
import '@material/mwc-dialog';
import '@material/mwc-textfield';
import '@material/mwc-textarea';
import '@material/mwc-icon-button';

import { sharedStyles } from '../shared-styles';
import { getClient } from '../graphql';
import {
  UPDATE_MODULE,
  CREATE_CONTENT,
  DELETE_MODULE,
  DELETE_CONTENT,
  UPDATE_CONTENT
} from '../graphql/queries';

export class LeapModule extends LitElement {
  static get properties() {
    return {
      module: {
        type: Object
      },
      editable: {
        type: Boolean
      },
      editingTitle: {
        type: Boolean
      },
      editingContent: {
        type: Object
      }
    };
  }

  constructor() {
    super();
    this.editingContent = {};
  }

  static get styles() {
    return [
      sharedStyles,
      css`
        .dialog-field {
          padding-top: 16px;
          padding-bottom: 16px;
        }

        .content-title {
          font-size: 18px;
        }

        hr {
          padding: 0;
          margin: 0;
        }

        .content-item {
          padding: 4px;
        }

        .action {
          --mdc-icon-button-size: 40px;
        }
      `
    ];
  }

  async createOrUpdateContent() {
    const client = await getClient();

    if (this.editingContent.id) {
      client.mutate({
        mutation: UPDATE_CONTENT,
        variables: {
          contentId: this.editingContent.id,
          content: {
            name: this.editingContent.name,
            description: this.editingContent.description,
            url: this.editingContent.url
          }
        }
      });
    } else {
      client.mutate({
        mutation: CREATE_CONTENT,
        variables: {
          moduleId: this.module.id,
          content: {
            name: this.editingContent.name,
            description: this.editingContent.description,
            url: this.editingContent.url
          }
        }
      });
    }

    window.location.reload();
  }

  async updateModule() {
    this.editingTitle = false;

    const client = await getClient();
    client.mutate({
      mutation: UPDATE_MODULE,
      variables: {
        moduleId: this.module.id,
        title: this.renameModule
      }
    });

    window.location.reload();
  }

  async deleteModule() {
    const client = await getClient();
    client.mutate({
      mutation: DELETE_MODULE,
      variables: {
        moduleId: this.module.id
      }
    });

    window.location.reload();
  }

  async deleteContent(contentId) {
    const client = await getClient();
    client.mutate({
      mutation: DELETE_CONTENT,
      variables: {
        contentId: contentId
      }
    });

    window.location.reload();
  }

  showContentDialog(existingContent) {
    this.editingContent = existingContent || {};
    this.shadowRoot.getElementById('create-content-dialog').open = true;
  }

  renderCreateContentDialog() {
    return html`
      <mwc-dialog
        id="create-content-dialog"
        .heading=${this.editingContent.id ? 'Edit content' : 'Add content'}
      >
        <div class="column" style="width: 500px; margin-top: 16px;">
          <mwc-textfield
            outlined
            class="dialog-field"
            label="Name"
            dialogInitialFocus
            .value=${this.editingContent.name || ''}
            @input=${e => (this.editingContent.name = e.target.value)}
          >
          </mwc-textfield>
          <mwc-textarea
            outlined
            class="dialog-field"
            label="Description"
            .value=${this.editingContent.description || ''}
            @input=${e => (this.editingContent.description = e.target.value)}
          >
          </mwc-textarea>
          <mwc-textfield
            outlined
            class="dialog-field"
            label="URL"
            .value=${this.editingContent.url || ''}
            @input=${e => (this.editingContent.url = e.target.value)}
          >
          </mwc-textfield>
        </div>

        <mwc-button
          slot="primaryAction"
          dialogAction="create"
          @click=${() => this.createOrUpdateContent()}
        >
          Create
        </mwc-button>
        <mwc-button slot="secondaryAction" dialogAction="cancel">
          Cancel
        </mwc-button>
      </mwc-dialog>
    `;
  }

  renderHeader() {
    return html`
      <div class="row" style="align-items: center;">
        <div style="flex: 1; align-items: center;" class="fill">
          ${this.renderTitle()}
        </div>
        ${this.renderToolbar()}
      </div>
    `;
  }

  renderTitle() {
    return html`
      <div class="row" style="align-items: center;">
        ${this.editable && this.editingTitle
          ? html`
              <mwc-textfield
                outlined
                @input=${e => (this.renameModule = e.target.value)}
                .value=${this.module.title}
              ></mwc-textfield>
            `
          : html`
              <span class="title"> ${this.module.title}</span>
            `}
      </div>
    `;
  }

  renderToolbar() {
    if (!this.editable) return html``;

    if (this.editingTitle)
      return html`
        <div class="row">
          <mwc-icon-button
            class="action"
            label="Save"
            icon="done"
            @click=${() => this.updateModule()}
          ></mwc-icon-button>
          <mwc-icon-button
            class="action"
            label="Cancel"
            icon="clear"
            @click=${() => (this.editingTitle = false)}
          ></mwc-icon-button>
        </div>
      `;

    return html`
      <div class="row">
        <mwc-icon-button
          class="action"
          label="Edit"
          icon="edit"
          @click=${() => (this.editingTitle = true)}
        ></mwc-icon-button>
        <mwc-icon-button
          slot="action-buttons"
          class="action"
          icon="add"
          label="Add content"
          @click=${() => this.showContentDialog()}
        ></mwc-icon-button>
        <mwc-icon-button
          slot="action-buttons"
          class="action"
          icon="delete"
          label="Delete module"
          @click=${() => this.deleteModule()}
        ></mwc-icon-button>
      </div>
    `;
  }

  renderContent(content, index) {
    return html`
      <mwc-list-item
        @click=${() => window.open(content.url)}
        class="content-item"
      >
        <div class="row">
          <div class="column" style="flex: 1;">
            <span class="content-title">${content.name}</span>
            <span class="fading">${content.description}</span>
          </div>

          ${this.editable
            ? html`
                <div class="row">
                  <mwc-icon-button
                    icon="edit"
                    label="Edit content"
                    @click=${e => {
                      e.stopPropagation();
                      this.showContentDialog(content);
                    }}
                  ></mwc-icon-button>
                  <mwc-icon-button
                    icon="delete"
                    label="Delete content"
                    @click=${e => {
                      e.stopPropagation();
                      this.deleteContent(content.id);
                    }}
                  ></mwc-icon-button>
                </div>
              `
            : html``}
        </div>
      </mwc-list-item>
      ${index !== this.module.contents.length - 1
        ? html`
            <hr style="opacity: 0.6" />
          `
        : html``}
    `;
  }

  render() {
    return html`
      ${this.renderCreateContentDialog()}

      <mwc-card class="fill">
        <div style="padding: 16px;" class="column">
          ${this.renderHeader()}
          ${this.module.contents.length === 0
            ? html`
                <leap-empty-placeholder
                  message="There are no contents in this module"
                ></leap-empty-placeholder>
              `
            : html`
                <mwc-list style="padding-top: 8px; padding-bottom: 8px;">
                  <div class="content-list">
                    ${this.module.contents.map((content, index) =>
                      this.renderContent(content, index)
                    )}
                  </div>
                </mwc-list>
              `}
        </div>
      </mwc-card>
    `;
  }
}
