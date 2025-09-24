<h2><a href="https://leetcode.com/problems/library-late-fee-calculator">3687. Library Late Fee Calculator</a></h2><h3>Easy</h3><hr><p>You are given an integer array <code>daysLate</code> where <code>daysLate[i]</code> indicates how many days late the <code>i<sup>th</sup></code> book was returned.</p>

<p>The penalty is calculated as follows:</p>

<ul>
	<li>If <code>daysLate[i] == 1</code>, penalty is 1.</li>
	<li>If <code>2 &lt;= daysLate[i] &lt;= 5</code>, penalty is <code>2 * daysLate[i]</code>.</li>
	<li>If <code>daysLate[i] &gt; 5</code>, penalty is <code>3 * daysLate[i]</code>.</li>
</ul>

<p>Return the total penalty for all books.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">daysLate = [5,1,7]</span></p>

<p><strong>Output:</strong> <span class="example-io">32</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><code>daysLate[0] = 5</code>: Penalty is <code>2 * daysLate[0] = 2 * 5 = 10</code>.</li>
	<li><code>daysLate[1] = 1</code>: Penalty is <code>1</code>.</li>
	<li><code>daysLate[2] = 7</code>: Penalty is <code>3 * daysLate[2] = 3 * 7 = 21</code>.</li>
	<li>Thus, the total penalty is <code>10 + 1 + 21 = 32</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">daysLate = [1,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><code>daysLate[0] = 1</code>: Penalty is <code>1</code>.</li>
	<li><code>daysLate[1] = 1</code>: Penalty is <code>1</code>.</li>
	<li>Thus, the total penalty is <code>1 + 1 = 2</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= daysLate.length &lt;= 100</code></li>
	<li><code>1 &lt;= daysLate[i] &lt;= 100</code></li>
</ul>
