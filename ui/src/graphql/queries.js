import gql from 'graphql-tag';

export const GET_COURSES = gql`
  query GetCourses($filter: String!) {
    courses(filter: $filter) {
      id
      title
      teacher_address
      students
    }
  }
`;

export const GET_COURSE_INFO = gql`
  query GetCourseInfo($courseId: String!) {
    myAddress
    course(courseId: $courseId) {
      id
      title
      students
      teacher_address
      modules {
        id
        title
        contents {
          id
          name
          description
          url
        }
      }
    }
  }
`;

export const CREATE_COURSE = gql`
  mutation CreateCourse($title: String!) {
    createCourse(title: $title) {
      id
      title
      teacher_address
    }
  }
`;

export const DELETE_COURSE = gql`
  mutation DeleteCourse($courseId: ID!) {
    deleteCourse(courseId: $courseId)
  }
`;

export const DELETE_MODULE = gql`
  mutation DeleteModule($moduleId: ID!) {
    deleteModule(moduleId: $moduleId)
  }
`;

export const DELETE_CONTENT = gql`
  mutation DeleteContent($contentId: ID!) {
    deleteContent(contentId: $contentId)
  }
`;

export const UPDATE_CONTENT = gql`
  mutation UpdateContent($contentId: ID, $content: ContentInput!) {
    updateContent(contentId: $contentId, content: $content) {
      id
      name
      description
      url
    }
  }
`;

export const CREATE_MODULE = gql`
  mutation CreateModule($courseId: ID!, $title: String!) {
    createModule(courseId: $courseId, title: $title) {
      id
      title
    }
  }
`;

export const UPDATE_MODULE = gql`
  mutation UpdateModule($moduleId: ID!, $title: String!) {
    updateModule(moduleId: $moduleId, title: $title) {
      id
      title
    }
  }
`;

export const CREATE_CONTENT = gql`
  mutation CreateContent($moduleId: ID!, $content: ContentInput!) {
    createContent(moduleId: $moduleId, content: $content) {
      id
      name
      description
      url
    }
  }
`;

export const ENROL_IN_COURSE = gql`
  mutation EnrolInCourse($courseId: ID!) {
    enrolInCourse(courseId: $courseId) {
      id
      title
      students
    }
  }
`;

export const GET_VALID_MEMBERS = gql`
  query GetValidMembers {
    validMembers
  }
`;
