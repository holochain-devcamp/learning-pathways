import { LitElement, html, css } from 'lit-element';

import '@authentic/mwc-card';
import '@material/mwc-list';
import '@material/mwc-list/mwc-list-item';
import '@material/mwc-dialog';
import '@material/mwc-textfield';
import '@material/mwc-textarea';
import '@material/mwc-icon';
import '@material/mwc-icon-button';

import { sharedStyles } from '../shared-styles';
import { getClient } from '../graphql';
import {
  UPDATE_MODULE,
  DELETE_MODULE,
  CREATE_CONTENT,
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

        .action {
          --mdc-icon-button-size: 40px;
        }
      `
    ];
  }

  async createOrUpdateContent() {
    const client = await getClient();

    if (this.editingContent.id) {
      await client.mutate({
        mutation: UPDATE_CONTENT,
        variables: {
          courseId: this.courseId,
          contentId: this.editingContent.id,
          content: {
            name: this.editingContent.name,
            description: this.editingContent.description,
            url: this.editingContent.url
          }
        }
      });
    } else {
      await client.mutate({
        mutation: CREATE_CONTENT,
        variables: {
          courseId: this.courseId,
          moduleId: this.module.id,
          content: {
            name: this.editingContent.name,
            description: this.editingContent.description,
            url: this.editingContent.url
          }
        }
      });
    }

    this.dispatchEvent(new CustomEvent('course-updated', { composed: true }));
  }

  async updateModule() {
    this.editingTitle = false;

    const client = await getClient();
    await client.mutate({
      mutation: UPDATE_MODULE,
      variables: {
        courseId: this.courseId,
        moduleId: this.module.id,
        title: this.renameModule
      }
    });

    this.dispatchEvent(new CustomEvent('course-updated', { composed: true }));
  }

  async deleteModule() {
    const client = await getClient();
    await client.mutate({
      mutation: DELETE_MODULE,
      variables: {
        courseId: this.courseId,
        moduleId: this.module.id
      }
    });

    this.dispatchEvent(new CustomEvent('course-updated', { composed: true }));
  }

  async deleteContent(contentId) {
    const client = await getClient();
    await client.mutate({
      mutation: DELETE_CONTENT,
      variables: {
        courseId: this.courseId,
        contentId: contentId
      }
    });

    this.dispatchEvent(new CustomEvent('course-updated', { composed: true }));
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
        ${this.editable
          ? html`
              <mwc-button
                slot="secondaryAction"
                dialogAction="cancel"
                label="Delete"
                @click=${e => {
                  e.stopPropagation();
                  this.deleteContent(this.editingContent.id);
                }}
              ></mwc-button>
            `
          : html``}
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
        hasMeta
        twoline
      >
        <span class="content-title">${content.name}</span>
        <span slot="secondary">${content.description}</span>

        ${this.editable
          ? html`
              <mwc-icon
                slot="meta"
                label="Edit content"
                @click=${e => {
                  e.stopPropagation();
                  this.showContentDialog(content);
                }}
                >edit</mwc-icon
              >
            `
          : html``}
      </mwc-list-item>
      ${index !== this.module.contents.length - 1
        ? html`
            <li divider padded role="separator"></li>
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
                  ${this.module.contents.map((content, index) =>
                    this.renderContent(content, index)
                  )}
                </mwc-list>
              `}
        </div>
      </mwc-card>
    `;
  }
}
