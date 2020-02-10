import { LitElement, html, css } from 'lit-element';

import '@authentic/mwc-circular-progress';
import '@material/mwc-button';
import '@material/mwc-icon-button';

import { sharedStyles } from '../shared-styles';
import { router } from '../router';
import { getClient } from '../graphql';
import {
  GET_COURSE_INFO,
  CREATE_MODULE,
  ENROL_IN_COURSE,
  DELETE_COURSE
} from '../graphql/queries';

export class LeapCourseDetail extends LitElement {
  static get properties() {
    return {
      courseId: {
        type: String
      },
      course: {
        type: Object
      }
    };
  }

  static get styles() {
    return [
      sharedStyles,
      css`
        .fab {
          position: absolute;
          bottom: 16px;
          right: 16px;
        }
      `
    ];
  }

  async loadCourse() {
    this.course = undefined;

    const client = await getClient();

    const result = await client.query({
      query: GET_COURSE_INFO,
      variables: {
        courseId: this.courseId
      }
    });

    this.course = result.data.course;
    this.myAddress = result.data.myAddress;
  }

  firstUpdated() {
    this.loadCourse();
  }

  updated(changedValues) {
    super.updated(changedValues);

    if (changedValues.get('courseId')) {
      this.loadCourse();
    }
  }

  async createModule() {
    const client = await getClient();

    const result = await client.mutate({
      mutation: CREATE_MODULE,
      variables: {
        courseId: this.courseId,
        title: this.moduleTitle
      },
      refetchQueries: [
        {
          query: GET_COURSE_INFO,
          variables: {
            courseId: this.courseId
          }
        }
      ]
    });

    window.location.reload();
  }

  renderCreateModuleDialog() {
    return html`
      <mwc-dialog id="create-module-dialog" heading="Create module">
        <mwc-textfield
          style="margin-top: 16px;"
          outlined
          label="Title"
          dialogInitialFocus
          @input=${e => (this.moduleTitle = e.target.value)}
        >
        </mwc-textfield>

        <mwc-button
          slot="primaryAction"
          dialogAction="create"
          @click=${() => this.createModule()}
        >
          Create
        </mwc-button>
        <mwc-button slot="secondaryAction" dialogAction="cancel">
          Cancel
        </mwc-button>
      </mwc-dialog>
    `;
  }

  renderModules() {
    if (this.course.modules.length === 0)
      return html`
        <leap-empty-placeholder
          message="There are no modules in this course"
        ></leap-empty-placeholder>
      `;

    return html`
      <div class="column">
        ${this.course.modules.map(
          module =>
            html`
              <leap-module
                .module=${module}
                .editable=${this.userIsTeacher()}
                style="padding-bottom: 24px;"
              ></leap-module>
            `
        )}
      </div>
    `;
  }

  userIsTeacher() {
    return this.myAddress === this.course.teacher_address;
  }

  async enrolInCourse() {
    const client = await getClient();

    await client.mutate({
      mutation: ENROL_IN_COURSE,
      variables: {
        courseId: this.courseId
      }
    });

    router.navigate('/home');
  }

  async deleteCourse() {
    const client = await getClient();

    await client.mutate({
      mutation: DELETE_COURSE,
      variables: {
        courseId: this.courseId
      }
    });

    router.navigate('/home');
  }

  renderCourseInfo() {
    return html`
      <mwc-card class="fill">
        <div class="row center-content" style="padding: 24px;">
          <div class="column fill">
            <span class="title" style="padding-bottom: 16px;"
              >${this.course.title}</span
            >
            <span>Taught by ${this.course.teacher_address}</span>
          </div>

          ${this.userIsTeacher()
            ? html`
                <div class="column">
                  <mwc-button
                    icon="add"
                    label="Add module"
                    raised
                    style="padding-bottom: 8px;"
                    @click=${() =>
                      (this.shadowRoot.getElementById(
                        'create-module-dialog'
                      ).open = true)}
                  ></mwc-button>

                  <mwc-button
                    icon="delete"
                    label="Delete course"
                    outlined
                    class="danger"
                    @click=${() => this.deleteCourse()}
                  ></mwc-button>
                </div>
              `
            : html`
                <mwc-button
                  icon="school"
                  label="Enrol in this course"
                  outlined
                  @click=${() => this.enrolInCourse()}
                ></mwc-button>
              `}
        </div>
      </mwc-card>
    `;
  }

  renderStudentsList() {
    if (this.course.students.length === 0)
      return html`
        <leap-empty-placeholder
          message="There are no students enrolled in this course"
        ></leap-empty-placeholder>
      `;

    return html`
      <mwc-list>
        ${this.course.students.map(
          student => html`
            <span>
              ${student}
            </span>
          `
        )}
      </mwc-list>
    `;
  }

  render() {
    if (!this.course)
      return html`
        <div class="column fill center-content" style="position: relative;">
          <mwc-circular-progress></mwc-circular-progress>
        </div>
      `;

    return html`
      ${this.renderCreateModuleDialog()}

      <div class="column">
        <mwc-top-app-bar>
          <mwc-icon-button
            icon="arrow_back"
            slot="navigationIcon"
            @click=${() => router.navigate('/home')}
          ></mwc-icon-button>
          <div slot="title">${this.course.title}</div>
        </mwc-top-app-bar>

        <div
          class="column"
          style="position: relative; padding: 16px; width: 1000px; align-self: center;"
        >
          ${this.renderCourseInfo()}

          <div class="row">
            <div class="column" style="flex: 3; padding-right: 24px;">
              <h3>Modules</h3>
              ${this.renderModules()}
            </div>

            <div class="column" style="flex: 1;">
              <h3>Students</h3>
              ${this.renderStudentsList()}
            </div>
          </div>
        </div>
      </div>
    `;
  }
}
