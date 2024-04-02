// import { createTheme } from '@mui/material/styles';
//
// const theme = createTheme({
//   palette: {
//     mode: 'dark',
//     primary: {
//       main: '#ff9900',
//
//     },
//     secondary: {
//       main: '#fff',
//     },
//   },
//   typography: {
//     fontFamily: 'Roboto, sans-serif',
//   },
// });
//
//
// export { theme };
//

import { createTheme } from '@mui/material/styles';

const theme = createTheme({
    palette: {
        mode: 'dark',
        primary: {
            main: '#ff9900',
          },
        secondary: {
            main: '#673ab7'
          },
        error: {
            main: '#d32f2f', // Red
          },
        warning: {
            main: '#ffeb3b', // Amber
          },
        info: {
            main: '#2196f3', // Blue
          },
        success: {
            main: '#4caf50', // Green
          },
        background: {
            default: '#121212', // Dark background
            paper: '#212121', // Slightly lighter background for cards/paper
          },
        text: {
            primary: '#fff', // White text
            secondary: 'rgba(255, 255, 255, 0.7)', // Light grey text
          },
      },
    typography: {
        fontFamily: 'Roboto, sans-serif',
      },
});

export { theme };
