import { LitElement, html, css } from 'lit-element';

import { USERNAME } from '../config';

import '@material/mwc-top-app-bar';
import '@material/mwc-dialog';
import '@material/mwc-textfield';
import '@material/mwc-button';
import '@material/mwc-fab';

import { sharedStyles } from '../shared-styles';
import { getClient } from '../graphql';
import { CREATE_COURSE } from '../graphql/queries';
import { router } from '../router';

const tabs = ['enrolled-courses', 'my-courses', 'all-courses'];

export class LeapDashboard extends LitElement {
  static get styles() {
    return [
      sharedStyles,
      css`
        mwc-card {
          margin: 24px;
        }

        .card-content {
          padding: 16px;
          min-height: 200px;
          display: flex;
          flex-direction: column;
        }

        .fab {
          position: absolute;
          right: 36px;
          bottom: 36px;
        }

        .title {
          font-size: 24px;
        }

        leap-courses-list {
          margin-top: 8px;
          flex: 1;
          display: flex;
        }
      `
    ];
  }

  firstUpdated() {
    this.activeTab = 0;
  }

  static get properties() {
    return {
      activeTab: {
        type: Number
      }
    };
  }

  renderCreateCourseDialog() {
    return html`
      <mwc-dialog id="create-course-dialog" heading="Create course">
        <mwc-textfield
          style="margin-top: 16px;"
          outlined
          label="Title"
          dialogInitialFocus
          @input=${e => (this.courseTitle = e.target.value)}
        >
        </mwc-textfield>

        <mwc-button
          slot="primaryAction"
          dialogAction="create"
          @click=${() => this.createCourse()}
        >
          Create
        </mwc-button>
        <mwc-button slot="secondaryAction" dialogAction="cancel">
          Cancel
        </mwc-button>
      </mwc-dialog>
    `;
  }

  async createCourse() {
    const client = await getClient();

    const result = await client.mutate({
      mutation: CREATE_COURSE,
      variables: {
        title: this.courseTitle
      }
    });

    router.navigate(`course/${result.data.createCourse.id}`);
  }

  render() {
    return html`
      ${this.renderCreateCourseDialog()}
      <div class="column fill" style="position: relative;">
        <mwc-top-app-bar>
          <div slot="title">LeaP ${USERNAME ? '/ ' + USERNAME : ''}</div>
        </mwc-top-app-bar>

        <div
          class="row fill"
          style="justify-content: center; align-items: start;"
        >
          <mwc-card>
            <div class="card-content">
              <span class="title">All courses</span>
              <leap-courses-list
                class="fill"
                filter="all-courses"
              ></leap-courses-list>
            </div>
          </mwc-card>

          <mwc-card>
            <div class="card-content">
              <span class="title">My courses</span>
              <leap-courses-list
                class="fill"
                filter="my-courses"
              ></leap-courses-list>
            </div>
          </mwc-card>

          <mwc-card>
            <div class="card-content">
              <span class="title">Enrolled courses</span>
              <leap-courses-list
                class="fill"
                filter="enrolled-courses"
              ></leap-courses-list>
            </div>
          </mwc-card>
        </div>

        <mwc-fab
          label="Create course"
          icon="add"
          extended
          class="fab"
          @click=${() =>
            (this.shadowRoot.getElementById(
              'create-course-dialog'
            ).open = true)}
        ></mwc-fab>
      </div>
    `;
  }
}
