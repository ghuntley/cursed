/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['Fira Code', 'Monaco', 'Consolas', 'monospace'],
      },
      animation: {
        'fade-in': 'fadeIn 0.5s ease-in-out',
        'slide-up': 'slideUp 0.3s ease-out',
        'pulse-soft': 'pulseSoft 2s infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        pulseSoft: {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.7' },
        },
      },
      colors: {
        primary: {
          50: '#eff6ff',
          100: '#dbeafe',
          200: '#bfdbfe',
          300: '#93c5fd',
          400: '#60a5fa',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
          800: '#1e40af',
          900: '#1e3a8a',
        },
      },
      typography: (theme) => ({
        DEFAULT: {
          css: {
            '--tw-prose-body': theme('colors.slate.700'),
            '--tw-prose-headings': theme('colors.slate.900'),
            '--tw-prose-lead': theme('colors.slate.600'),
            '--tw-prose-links': theme('colors.blue.600'),
            '--tw-prose-bold': theme('colors.slate.900'),
            '--tw-prose-counters': theme('colors.slate.500'),
            '--tw-prose-bullets': theme('colors.slate.300'),
            '--tw-prose-hr': theme('colors.slate.200'),
            '--tw-prose-quotes': theme('colors.slate.900'),
            '--tw-prose-quote-borders': theme('colors.slate.200'),
            '--tw-prose-captions': theme('colors.slate.500'),
            '--tw-prose-code': theme('colors.slate.900'),
            '--tw-prose-pre-code': theme('colors.slate.200'),
            '--tw-prose-pre-bg': theme('colors.slate.800'),
            '--tw-prose-th-borders': theme('colors.slate.300'),
            '--tw-prose-td-borders': theme('colors.slate.200'),
            '--tw-prose-invert-body': theme('colors.slate.300'),
            '--tw-prose-invert-headings': theme('colors.white'),
            '--tw-prose-invert-lead': theme('colors.slate.400'),
            '--tw-prose-invert-links': theme('colors.blue.400'),
            '--tw-prose-invert-bold': theme('colors.white'),
            '--tw-prose-invert-counters': theme('colors.slate.400'),
            '--tw-prose-invert-bullets': theme('colors.slate.600'),
            '--tw-prose-invert-hr': theme('colors.slate.700'),
            '--tw-prose-invert-quotes': theme('colors.slate.100'),
            '--tw-prose-invert-quote-borders': theme('colors.slate.700'),
            '--tw-prose-invert-captions': theme('colors.slate.400'),
            '--tw-prose-invert-code': theme('colors.white'),
            '--tw-prose-invert-pre-code': theme('colors.slate.300'),
            '--tw-prose-invert-pre-bg': 'rgb(0 0 0 / 50%)',
            '--tw-prose-invert-th-borders': theme('colors.slate.600'),
            '--tw-prose-invert-td-borders': theme('colors.slate.700'),
          },
        },
      }),
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
  ],
}
