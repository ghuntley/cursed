# Contributing to CURSED Interactive Documentation

Welcome to the CURSED community! We're excited to have you contribute to making CURSED more accessible and learnable for developers worldwide.

## 🎯 Ways to Contribute

### 📚 Documentation Contributions
- **Tutorial Content**: Write or improve interactive tutorials
- **Code Examples**: Add runnable examples and exercises
- **API Documentation**: Improve function and module documentation
- **Migration Guides**: Help developers transition from other languages
- **Best Practices**: Share patterns and architectural guidance

### 🛠️ Technical Contributions
- **Interactive Features**: Improve the playground and editor
- **Search and Navigation**: Enhance documentation discoverability
- **Performance**: Optimize loading and interaction speed
- **Accessibility**: Make documentation accessible to all users
- **Mobile Experience**: Improve mobile documentation experience

### 🎥 Video and Media Contributions
- **Video Tutorials**: Create educational video content
- **Screencasts**: Record development workflow demonstrations
- **Diagrams and Illustrations**: Visual explanations of concepts
- **Interactive Examples**: Animated or interactive learning content

### 🌐 Community Support
- **Question Answering**: Help other users in forums and Discord
- **Code Reviews**: Review community-submitted examples
- **Translation**: Translate documentation to other languages
- **Event Organization**: Organize meetups, workshops, or conferences

## 🚀 Getting Started

### 1. Set Up Development Environment

```bash
# Clone the repository
git clone https://github.com/ghuntley/cursed.git
cd cursed/docs/interactive

# Install dependencies
npm install

# Start development server
npm run dev

# Open browser to http://localhost:5173
```

### 2. Understand the Project Structure

```
docs/interactive/
├── webapp/              # React-based interactive documentation app
│   ├── src/
│   │   ├── components/  # Reusable UI components
│   │   ├── pages/       # Documentation pages
│   │   ├── contexts/    # React contexts for state management
│   │   └── utils/       # Utility functions
├── tutorials/           # Interactive tutorial content
├── api-docs/           # API documentation with examples
├── pathways/           # Learning pathways for different users
├── migration/          # Language migration guides
├── patterns/           # Best practices and design patterns
├── videos/             # Video content and transcripts
└── community/          # Community guidelines and resources
```

### 3. Choose Your Contribution Type

#### 📝 Writing Documentation
1. **Find an Issue**: Look for issues labeled `documentation`, `tutorial`, or `help-wanted`
2. **Propose New Content**: Create an issue describing what you want to add
3. **Follow Style Guide**: Use our [writing style guide](writing-style-guide.md)
4. **Use Templates**: Start with our [content templates](templates/)

#### 💻 Technical Development
1. **Check Technical Issues**: Look for `frontend`, `backend`, or `enhancement` labels
2. **Set up Environment**: Follow the setup instructions above
3. **Run Tests**: `npm test` for unit tests, `npm run test:e2e` for integration tests
4. **Follow Code Standards**: Use ESLint and Prettier configurations

## 📋 Contribution Guidelines

### Writing Quality Standards

#### Content Requirements
- **Accuracy**: All code examples must run without errors
- **Completeness**: Tutorials should have clear learning objectives and outcomes
- **Accessibility**: Use clear language, avoid jargon, provide definitions
- **Interactivity**: Include runnable code examples where possible
- **Currency**: Keep content up-to-date with latest CURSED features

#### Writing Style
- **Tone**: Friendly, encouraging, professional
- **Structure**: Use clear headings, bullet points, and numbered lists
- **Examples**: Provide practical, real-world examples
- **Progression**: Build complexity gradually
- **Feedback**: Include exercises and checkpoints

### Code Quality Standards

#### Frontend (React/TypeScript)
```typescript
// Use functional components with hooks
const ExampleComponent: React.FC<Props> = ({ prop1, prop2 }) => {
  const [state, setState] = useState<StateType>(initialState);
  
  // Clear, descriptive function names
  const handleUserAction = useCallback(() => {
    // Implementation
  }, [dependencies]);
  
  return (
    <div className="semantic-class-name">
      {/* JSX content */}
    </div>
  );
};
```

#### CURSED Code Examples
```cursed
# All examples must be complete and runnable
yeet "vibez"
yeet "testz"

# Use descriptive variable names
sus user_name tea = "Alice"
sus age_in_years drip = 25

# Include error handling where appropriate
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

# Provide test cases for complex examples
slay test_divide() {
    testz.assert_eq(divide(10, 2), 5)
    testz.assert_error(divide(10, 0))
}
```

### Documentation Templates

#### Tutorial Template
```markdown
# Tutorial Title

Brief description of what the tutorial covers and learning objectives.

## Learning Objectives
- Objective 1
- Objective 2
- Objective 3

## Prerequisites
- Required knowledge
- Tools needed

## Step 1: Introduction
Explanation of concept...

<interactive-editor>
# Runnable code example
yeet "vibez"
vibez.spill("Hello, World!")
</interactive-editor>

## Step 2: Building Understanding
More detailed explanation...

### Try It Yourself
<interactive-editor>
# Exercise for the reader
</interactive-editor>

## Summary
- What we learned
- Key takeaways
- Next steps

## Exercises
1. Exercise 1
2. Exercise 2

## Additional Resources
- [Related documentation](link)
- [Video tutorial](link)
```

#### API Documentation Template
```markdown
# Module/Function Name

Brief description of purpose and use cases.

## Syntax
```cursed
function_signature
```

## Parameters
- `param1` (type): Description
- `param2` (type): Description

## Return Value
- Returns `type`: Description

## Examples

### Basic Usage
<interactive-editor>
yeet "module"
sus result type = function_call(param1, param2)
vibez.spill(result)
</interactive-editor>

### Advanced Usage
<interactive-editor>
# More complex example
</interactive-editor>

## Error Handling
Possible errors and how to handle them.

## See Also
- [Related function](link)
- [Related concept](link)
```

## 🔄 Contribution Process

### 1. Planning Phase
1. **Browse Issues**: Check [GitHub issues](https://github.com/ghuntley/cursed/issues) for existing work
2. **Discuss Ideas**: Use [GitHub Discussions](https://github.com/ghuntley/cursed/discussions) for new ideas
3. **Create Issue**: For new features or content, create an issue first
4. **Get Feedback**: Wait for maintainer feedback before starting work

### 2. Development Phase
1. **Fork Repository**: Create your own fork of the project
2. **Create Branch**: Use descriptive branch names (`feature/tutorial-async-programming`)
3. **Make Changes**: Follow the guidelines above
4. **Test Thoroughly**: Ensure all code examples work
5. **Write Tests**: Add tests for new features

### 3. Review Phase
1. **Self Review**: Review your own changes before submitting
2. **Create Pull Request**: Use the PR template provided
3. **Address Feedback**: Respond to reviewer comments promptly
4. **Update Documentation**: Update related documentation if needed

### Pull Request Guidelines

#### PR Title Format
- `docs: add tutorial on async programming`
- `feat: improve playground editor performance`
- `fix: resolve mobile navigation issue`
- `video: add concurrency screencast`

#### PR Description Template
```markdown
## Description
Brief description of changes made.

## Type of Change
- [ ] Documentation update
- [ ] New tutorial/guide
- [ ] Code example addition
- [ ] Bug fix
- [ ] New feature
- [ ] Video/media content

## Testing
- [ ] All code examples run successfully
- [ ] Interactive elements work correctly
- [ ] Mobile responsiveness checked
- [ ] Accessibility tested

## Screenshots (if applicable)
Add screenshots of UI changes.

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
```

## 🏆 Recognition and Rewards

### Contributor Levels
- **First-Time Contributor**: Welcome package and mentoring
- **Regular Contributor**: Featured in contributor spotlight
- **Core Contributor**: Access to maintainer discussions
- **Community Leader**: Speaking opportunities at CURSED events

### Benefits
- **Swag**: CURSED stickers, t-shirts, and branded items
- **Recognition**: Contributor wall of fame
- **Networking**: Access to CURSED developer community
- **Learning**: Early access to new features and content
- **Career**: Open source contributions for portfolio

### Special Programs
- **Student Contributors**: Mentorship and career guidance
- **Content Creator Program**: Support for video/blog creators
- **Translation Team**: Coordinate internationalization efforts
- **Accessibility Team**: Improve inclusive design

## 📞 Getting Help

### Community Channels
- **Discord**: [#contributors channel](https://discord.gg/cursed-lang) for real-time help
- **GitHub Discussions**: For long-form questions and planning
- **Office Hours**: Weekly sessions with maintainers (Wednesdays 3 PM UTC)
- **Mentorship**: Paired with experienced contributors for guidance

### Common Questions

#### "I'm new to open source, where do I start?"
1. Look for issues labeled `good-first-issue`
2. Join our Discord and introduce yourself
3. Start with documentation improvements
4. Attend virtual office hours for guidance

#### "I want to add a new tutorial, what's the process?"
1. Create a GitHub issue describing the tutorial
2. Get feedback from maintainers
3. Use the tutorial template provided
4. Submit a PR with the content

#### "How do I test interactive code examples?"
1. Run the development server (`npm run dev`)
2. Navigate to your content in the browser
3. Test all interactive editors manually
4. Verify code runs without errors

#### "Can I contribute in languages other than English?"
Yes! We welcome translations and localized content:
1. Check existing translation efforts
2. Join the translation team on Discord
3. Follow the translation guidelines
4. Coordinate with other translators

## 🛡️ Code of Conduct

### Our Standards
- **Respectful**: Treat all community members with respect
- **Inclusive**: Welcome people of all backgrounds and skill levels
- **Constructive**: Provide helpful, actionable feedback
- **Collaborative**: Work together toward common goals
- **Patient**: Remember everyone is learning

### Unacceptable Behavior
- Harassment or discrimination of any kind
- Toxic or aggressive communication
- Spam or off-topic content
- Sharing others' private information
- Plagiarism or unauthorized use of content

### Reporting Issues
If you experience or witness unacceptable behavior:
1. **Direct Contact**: Reach out to maintainers privately
2. **Email**: [conduct@cursedlang.org](mailto:conduct@cursedlang.org)
3. **Anonymous Form**: [Report misconduct anonymously](https://forms.cursedlang.org/conduct)

## 📜 Legal Information

### Content License
- **Documentation**: Licensed under [Creative Commons CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/)
- **Code Examples**: Licensed under [MIT License](https://opensource.org/licenses/MIT)
- **Videos/Media**: Licensed under [Creative Commons CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)

### Contributor Agreement
By contributing to CURSED documentation:
1. You grant us license to use your contributions
2. You confirm you have rights to contribute the content
3. You agree to the project's code of conduct
4. You understand contributions are public and attributed

### Attribution
- All contributors are credited in the project
- Major contributors featured on the website
- Video contributors credited in video descriptions
- Translation teams recognized in localized versions

---

## 🎉 Welcome to the Community!

Thank you for your interest in contributing to CURSED! Your contributions help make programming more accessible and enjoyable for developers worldwide.

**Questions?** Don't hesitate to reach out through any of our community channels. We're here to help and excited to work with you!

**Happy Contributing!** 🚀

---

*Last updated: January 2025*
*Version: 1.0*
