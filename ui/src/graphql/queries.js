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
    deleteCourse(courseId: $courseId) {
      courses(filter: "get_all_courses") {
        id
        title
        teacher_address
        students
      }
    }
  }
`;

export const DELETE_MODULE = gql`
  mutation DeleteModule($courseId: ID!, $moduleId: ID!) {
    deleteModule(courseId: $courseId, moduleId: $moduleId) {
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

export const DELETE_CONTENT = gql`
  mutation DeleteContent($courseId: ID!, $contentId: ID!) {
    deleteContent(courseId: $courseId, contentId: $contentId) {
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

export const UPDATE_CONTENT = gql`
  mutation UpdateContent(
    $courseId: ID!
    $contentId: ID
    $content: ContentInput!
  ) {
    updateContent(contentId: $contentId, content: $content) {
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

export const CREATE_MODULE = gql`
  mutation CreateModule($courseId: ID!, $title: String!) {
    createModule(courseId: $courseId, title: $title) {
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

export const UPDATE_MODULE = gql`
  mutation UpdateModule($courseId: ID!, $moduleId: ID!, $title: String!) {
    updateModule(courseId: $courseId, moduleId: $moduleId, title: $title) {
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

export const CREATE_CONTENT = gql`
  mutation CreateContent(
    $courseId: ID!
    $moduleId: ID!
    $content: ContentInput!
  ) {
    createContent(courseId: $courseId, moduleId: $moduleId, content: $content) {
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

export const ENROL_IN_COURSE = gql`
  mutation EnrolInCourse($courseId: ID!) {
    enrolInCourse(courseId: $courseId) {
      id
      title
      students
    }
  }
`;
