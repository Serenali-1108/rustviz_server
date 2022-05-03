'use strict';

// namespace
const create = React.createElement;

// Global variables
// temporarily stores all of the user's saved and unsaved answers
var ans_vec = [];
var saved_answers = [];
var saved_text = [];

// Child class
// Stores question info:
//      -- prompt
//      -- answers/choices
//      -- related svg's filename
class Question extends React.Component {
  /*    Display feed   */
  constructor(props) {
      // Initialize mutable state
      super(props);
      this.state = {
        // from REST API
        qid: 0,
        filename: '',
        prompt: '',
        choices: [],
        contains_fr: false,
        // state
        selected: -1,
        text: '',
        hoverTime: 0
      };

      // fn bindings
      this.handleChange = this.handleChange.bind(this);
      this.handleSubmit = this.handleSubmit.bind(this);
      this.debounceSubmit = debounce(this.handleSubmit, 1000);
      this.handleHover = this.handleHover.bind(this);
      this.startStopwatch = this.startStopwatch.bind(this);
      this.endStopwatch = this.endStopwatch.bind(this);
  }
  
  // On DOM mount, fetch user's last unanswered question
  componentDidMount() {
      // This line automatically assigns this.props.url to the const variable url
      const { url } = this.props;

    // Call REST API to get Question questions
    fetch(url, { credentials: 'same-origin' })
      .then((response) => {
        if (!response.ok) throw Error(response.statusText);
        return response.json();
      })
      .then((data) => {
        this.setState({
          qid: data.qid,
          filename: data.filename,
          prompt: data.prompt,
          choices: data.choices,
          contains_fr: data.contains_free_response,
          // initialize to last saved answer
          select: ans_vec[data.qid],
          text: saved_text[data.qid]
        });
      })
      .catch((error) => console.log(error));
  }

  // store current answer selection as int
  handleChange(e) {
    if (e.target.name === 'free_response') {
      let response = e.target.value;
      this.setState({
        text: response
      });
      // temporarily save text
      const { qid } = this.state;
      saved_text[qid] = response;
      // new answer unsaved in db
      saved_answers[qid] = false;
      this.debounceSubmit();
    }
    else { // === 'choices'
      let val = Number(e.target.value);

      //modify curr answers vec
      const { qid } = this.state;
      ans_vec[qid] = val;
      // new answer unsaved
      saved_answers[qid] = false;

      this.setState({
        selected: val
      });
      this.props.postResponse(val, this.state.text);
    }
  }

  // lift to parent class to handle answer submission
  handleSubmit() {
      const { selected } = this.state;
      const { qid } = this.state;

      // fetch clicked or previously selected answer
      let answer_id = selected != -1 ? selected : ans_vec[qid];

      if (answer_id < 0) {
        return;
      }

      const { postResponse } = this.props;
      const { text } = this.state;

      postResponse(answer_id, text);
  }

  // Calls helpers from helpers.js
  handleHover() {
    const { filename } = this.state;
    helpers(filename);
  }

  // begin hover timer
  startStopwatch() {
    this.setState({
      hoverTime: Date.now()
    });
  }

  // mouseleave is fired when the pointer has exited
  // the element and all of its descendants
  endStopwatch() {
    const { hoverTime } = this.state;
    const { postResponse } = this.props;

    let time_elapsed = Date.now() - hoverTime;
    postResponse(...[,,], time_elapsed);
  }
  
  render() {
    const { filename } = this.state;
    const { prompt } = this.state;
    const { choices } = this.state;
    const { qid } = this.state;
    const { contains_fr } = this.state;

    // conditionally render free response
    let textbox;
    if (contains_fr) {
      textbox = create('textarea', {
        class: 'form_in',
        rows: '7',
        cols: '25',
        name: 'free_response',
        placeholder: 'Your response.',
        required: true
      }, saved_text[qid]);
    }

    return create('div', null, 
      create('div', {
        class: 'flex-container vis_block'//,
        // onMouseEnter: this.startStopwatch,
        // onMouseLeave: this.endStopwatch
      },
        create('object', {
          type: 'image/svg+xml',
          class: filename,
          data: `/book/assets/img/${filename}.svg`
        })
      ),
      // prompt
      create('h3', null, `${prompt}`),
      // displayed whether answer was saved or not
      saved_answers[qid] === false ?
        // if answer is not entered yet or is during saving, display a transparent placeholder
        create('p', { style: { fontSize: '14px', color: 'rgba(0, 0, 0, 0)' } }, '✅')
        // if answer is saved, show message
        : create('p', { style: { fontSize: '14px' } }, '✅ Saved All Answers.'),
      // multiple choice form
      create('form', { onChange: this.handleChange },
        // create choices/input buttons
        create('div', {},
          choices.map(item => 
          create('div', { class: 'form_in' },
            create('label', null, 
                create('input', {
                  required: true,
                  // display previously selected answer
                  checked: ans_vec[qid] === item.id ? true : false,
                  type: 'radio',
                  name: 'choices',
                  value: item.id
                }),
            item.text)
          ))
        ),
        create('div', { class: 'form_panel'},
          // textbox for free response
          textbox
        )
      )
    );
  }
}

// Parent class
// Stores:
//      -- user's current question
//      -- time spent on current question
class Quiz extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
        total: 0,
        curr_qid: 0,
        ques_url: '',
        startTime: 0
    };

    // define fn bindings
    this.postResponse = this.postResponse.bind(this);
    this.handleNext = this.handleNext.bind(this);
    this.handlePrevious = this.handlePrevious.bind(this);
  }

  // on mount
  //      (1) retrieve last unanswered question
  //      (2) begin stopwatch
  componentDidMount() {
    const { url } = this.props;

    fetch(url, { credentials: 'same-origin' })
    .then((response) => {
        if (!response.ok) {
          // if not logged in, redirect home
          if (response.status === 401) {
            window.location = '/';
            alert('Please log in! Redirecting to home page.');
          }
          throw Error(response.statusText);
        }
        else return response.json()
    })
    .then((data) => {
        // populate ans, saved arrays
        data.saved_ans_vec.map(item => saved_answers.push(item === -2 ? false : true));
        ans_vec = data.saved_ans_vec;
        saved_text = data.saved_free_res;

        this.setState({
            total: data.total,
            startTime: Date.now(),
            ques_url: data.url,
            curr_qid: data.current,
        });
    })
    .catch((error) => console.log(error));
  }

  
  // Switch to next question
  handleNext(e) {
    e.preventDefault();

    // save the response!
    this.refs.quiz.handleSubmit();

    const { curr_qid } = this.state;
    const { total } = this.state;
    let next = curr_qid >= total-1 ? curr_qid : curr_qid + 1;
    this.setState({
      curr_qid: next,
      startTime: Date.now()
    });

    // update time_elapsed
    this.postResponse();
  }

  // Switch to previous question
  handlePrevious(e) {
    e.preventDefault();

    // save the response!
    this.refs.quiz.handleSubmit();

    const { curr_qid } = this.state;
    let prev = curr_qid <= 0 ? 0 : curr_qid - 1;
    this.setState({
      curr_qid: prev,
      startTime: Date.now()
    });

    // update time_elapsed
    this.postResponse();
  }

  // Post response to server
  postResponse(a_id = null, text = null, hoverTimeElapsed = 0) {
    const { curr_qid } = this.state;
    const { startTime } = this.state;
    let timeElapsed = Date.now() - startTime;

    // only update time_elapsed, hover_time_elapsed
    let update = false;
    let req_body = {};
    if (a_id === null) {
      req_body = {
        ques_id: curr_qid,
        time_elapsed_question: timeElapsed,
        time_elapsed_hover: hoverTimeElapsed
      };
    }
    else {
      req_body = {
          ques_id: curr_qid,
          ans_id: a_id,
          free_response: text,
          time_elapsed_question: timeElapsed,
          time_elapsed_hover: hoverTimeElapsed
      };
      update = true;
    }

    fetch('/submit',
      {
        method: 'POST',
        credentials: 'same-origin',
        headers: { 'Accept': 'application/json', 'Content-Type': 'application/json' },
        body: JSON.stringify(req_body)
      }
    )
    .then((response) => {
      if (!response.ok) throw Error(response);
      else {
        if (update) {
          // update current answers
          ans_vec[curr_qid] = a_id;
          saved_answers[curr_qid] = true;
        }

        this.setState({
            startTime: Date.now(),
        });
      }
    })
    .catch((error) => console.log(error));
  }

  render() {
    //   const { ques_url } = this.state;
    const { url } = this.props;
    const { curr_qid } = this.state;
    const { total } = this.state;

    // generate previous and next buttons
    let prev, next;
    if (curr_qid > 0) {
      prev = create('button', {
        class: 'btn btn-orange',
        onClick: this.handlePrevious
      }, '◀ previous');
    }
    if (curr_qid < total-1) {
      next = create('button', {
        class: 'btn btn-orange',
        onClick: this.handleNext
      }, 'next ▶');
    }
    let btns = create('div', { class: 'flex-btns' },
      prev,
      next
    );

    return create('div', null,
      create(
        // Child class
        Question, {
          // props
          key: curr_qid,
          url: `${url + curr_qid}/`,
          ans_vec: ans_vec,
          postResponse: this.postResponse,
          ref: 'quiz'
        }
      ),
      btns
    );
  }
}

// source: https://levelup.gitconnected.com/debounce-in-javascript-improve-your-applications-performance-5b01855e086
// Returns a function, that, as long as it continues to be invoked, will not
// be triggered. The function will be called after it stops being called for
// `wait` milliseconds.
const debounce = (func, wait) => {
  let timeout;

  // This is the function that is returned and will be executed many times
  // We spread (...args) to capture any number of parameters we want to pass
  return function executedFunction(...args) {

    // The callback function to be executed after 
    // the debounce time has elapsed
    const later = () => {
      // indicate the debounce ended
      clearTimeout(timeout);
      
      // Execute the callback
      func(...args);
    };
    // This will reset the waiting every function execution.
    // This is the step that prevents the function from
    // being executed because it will never reach the 
    // inside of the previous setTimeout  
    clearTimeout(timeout);
    
    // Restart the debounce waiting period.
    // setTimeout returns a truthy value (it differs in web vs Node)
    timeout = setTimeout(later, wait);
  };
};

const domContainer = document.querySelector('#quiz');
ReactDOM.render(create(Quiz, {url: '/question/'}), domContainer);