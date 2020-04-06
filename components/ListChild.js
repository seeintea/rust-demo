import React from 'react';
import styled from 'styled-components'

class ListChild extends React.Component {

	constructor(props){
		super(props);
	}

	ListChild = styled.div`
		position: relative;
		padding: 3rem 1.2rem;
		& .opacity {
			position: absolute;
			width: 11rem;
			height: 11rem;
			line-height: 11rem;
			text-align: center;
			color: rgba(0,0,0,0.1);
			font-size: 11rem;
			font-weight: 600;
			top: 1.9rem;
			left: -2.1rem;
			cursor: default;
		}
		& .title {
			font-size: 3rem;
			font-weight: bold;
			color: #333;
			margin: 3rem 0 .5rem 0;
		}
		& .datetime {
			font-size: 0.9rem;
			color: #666;
			margin: 0 0 1rem;
		}
		& .intro {
			font-size: .9rem;
		}
		&:after {
			content: "";
			height: 1px;
			width: 4rem;
			position: absolute;
			bottom: 0;
			left: 50%;
			margin-left: -25px;
			background: rgba(51,51,51,0.2);
		}
	`;

	render() {
		return (
			<this.ListChild>
				<div className='opacity'>
					{ this.props.title.slice(0,1) }
				</div>
				<p className='title'>{ this.props.title }</p>
				<p className='datetime'>{ this.props.date }</p>
				<div className='intro'>
					{ this.props.intro }
				</div>
			</this.ListChild>
		)
	}
}

export default ListChild