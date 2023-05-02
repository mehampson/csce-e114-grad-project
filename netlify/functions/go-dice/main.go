package main

import (
	"fmt"
	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"math/rand"
	"strconv"
	"time"
)

/* As an FYI, I've never written Go before today and absolutely do not assert this function is well-written. */

func handler(request events.APIGatewayProxyRequest) (*events.APIGatewayProxyResponse, error) {
	/* Parse the query strings. 
	 * We'll need to convert them both from strings to ints, and can handle `count`` in place. */
	count, count_err := strconv.Atoi(request.QueryStringParameters["count"])
	s := request.QueryStringParameters["sides"]

	/* The `sides` query string will be in the format 'dX'. We'll use a switch pattern since we know what values to expect,
	 * We could have just stripped the 'd' and done Atoi(), but I'm choosing not to implement this to handle arbitray input */
	var sides int
	switch s {
	case "d2":
		sides = 2
	case "d4":
		sides = 4
	case "d6":
		sides = 6
	case "d8":
		sides = 8
	case "d10":
		sides = 10
	case "d12":
		sides = 12
	case "d20":
		sides = 20
	default:
		// We're actually going to treat this as an error in a moment
		sides = -1
	}

	/* Our rule is the query strings have to be castable to ints. If we couldn't, return a 400. */
	if sides == -1 || count_err != nil {
		return &events.APIGatewayProxyResponse{
			StatusCode: 400,
			Body:       "Invalid dice roll request",
		}, nil
	} else {
		/* Okay, we know how to interpret this request. Let's roll our dice. */

		// Initialize the RNG
		rand.New(rand.NewSource(time.Now().UnixNano()))

		/* A slice to collect our rolls */
		var rolls []int
		var sum int

		/* Roll our dice and track the sum along the way */
		for i := 0; i < count; i++ {
			roll := rand.Intn(sides) + 1
			rolls = append(rolls, roll)
			sum += roll
		}

		/* We're returning an HTML fragment here, and we'll insert it on the page with HTMX */
		return &events.APIGatewayProxyResponse{
			StatusCode: 200,
			Headers: map[string]string{"Content-Type": "text/html"},
			Body:       fmt.Sprintf("<li class='go-rolls has-text-primary-dark'>[Go] You rolled %dd%d: %v = %d</li>", count, sides, rolls, sum),
		}, nil
	}

}

func main() {
	lambda.Start(handler)
}
