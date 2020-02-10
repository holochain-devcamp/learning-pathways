import { css } from 'lit-element';

export const sharedStyles = css`
  .column {
    display: flex;
    flex-direction: column;
  }

  .row {
    display: flex;
    flex-direction: row;
  }

  .fill {
    flex: 1;
    height: 100%;
    width: 100%;
  }

  .center-content {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .fading {
    opacity: 0.7;
  }

  .medium-padding {
    padding: 16px;
  }

  .title {
    font-size: 20px;
    font-weight: bold;
  }

  .danger {
    --mdc-theme-primary: red;
  }
`;
