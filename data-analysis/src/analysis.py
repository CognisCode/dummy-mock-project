import pandas as pd
from sqlalchemy import create_engine
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import seaborn as sns
import numpy as np

sns.set_theme(style="whitegrid", rc={'axes.facecolor':'#121212', 'grid.color': '#444444'})

engine = create_engine('postgresql://yassin:yassin@localhost:5432/simulation_data')

query = """
    SELECT timestamp, smart_score, high_score, close_score, genetic_score,
           smart_start_x, smart_start_y, 
           high_start_x, high_start_y, 
           close_start_x, close_start_y,
           genetic_start_x, genetic_start_y
    FROM simulation_data
    ORDER BY timestamp DESC
    LIMIT 5000;
"""

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(8, 8))  

smart_line, = ax1.plot([], [], label='Smart Score', color='black', lw=2) 
high_line, = ax1.plot([], [], label='High Score', color='red', lw=2)  
close_line, = ax1.plot([], [], label='Close Score', color='magenta', lw=2)  
genetic_line, = ax1.plot([], [], label='Genetic Score', color='blue', lw=2)  

smart_eucl, = ax2.plot([], [], label='Smart Distance', color='black', lw=2)  
high_eucl, = ax2.plot([], [], label='High Distance', color='red', lw=2) 
close_eucl, = ax2.plot([], [], label='Close Distance', color='magenta', lw=2)  
genetic_eucl, = ax2.plot([], [], label='Genetic Distance', color='blue', lw=2)  

ax1.set_title('Streaming Data of Scores', fontsize=20, color='black', weight='bold') 
ax1.set_xlabel('Data Points', fontsize=14, color='black') 
ax1.set_ylabel('Score', fontsize=14, color='black') 
ax1.legend(loc="upper left", fontsize=12, facecolor='#F5F5DC', edgecolor='black', labelcolor='black') 

ax1.set_facecolor('#F5F5DC')  
ax1.grid(True, color='#CCCCCC', linestyle='--', linewidth=0.7)  

ax1.tick_params(axis='x', colors='black')  
ax1.tick_params(axis='y', colors='black')  

ax1.spines['bottom'].set_color('black')
ax1.spines['left'].set_color('black')

ax2.set_title('Distance to other chasers', fontsize=20, color='black', weight='bold') 
ax2.set_xlabel('Data Points', fontsize=14, color='black')  
ax2.set_ylabel('Distance', fontsize=14, color='black')  
ax2.legend(loc="upper left", fontsize=12, facecolor='#F5F5DC', edgecolor='black', labelcolor='black')  

ax2.set_facecolor('#F5F5DC') 
ax1.grid(True, color='#CCCCCC', linestyle='--', linewidth=0.7)  

ax2.tick_params(axis='x', colors='black')  
ax2.tick_params(axis='y', colors='black')  

ax2.spines['bottom'].set_color('black')
ax2.spines['left'].set_color('black')

smart_scores = []
high_scores = []
close_scores = []
genetic_scores = []

smart_distance = []
high_distance = []
close_distance = []
genetic_distance = []

ax1.set_ylim(0, 15000)
ax2.set_ylim(0, 15000)

def init():
    ax1.set_xlim(0, 5000) 
    ax2.set_xlim(0, 5000) 
    return smart_line, high_line, close_line, genetic_line

def update(frame):
    df = pd.read_sql(query, engine)

    latest_smart = df['smart_score'].iloc[::-1].values 
    latest_high = df['high_score'].iloc[::-1].values
    latest_close = df['close_score'].iloc[::-1].values
    latest_genetic = df['genetic_score'].iloc[::-1].values


    latest_smart_start_x = df['smart_start_x'].iloc[::-1].values
    latest_smart_start_y = df['smart_start_y'].iloc[::-1].values
    latest_high_start_x = df['high_start_x'].iloc[::-1].values
    latest_high_start_y = df['high_start_y'].iloc[::-1].values
    latest_close_start_x = df['close_start_x'].iloc[::-1].values
    latest_close_start_y = df['close_start_y'].iloc[::-1].values
    latest_genetic_start_x = df['genetic_start_x'].iloc[::-1].values
    latest_genetic_start_y = df['genetic_start_y'].iloc[::-1].values

    smart_scores.extend(latest_smart)
    high_scores.extend(latest_high)
    close_scores.extend(latest_close)
    genetic_scores.extend(latest_genetic)


    smart_scores[:] = smart_scores[-5000:]  
    high_scores[:] = high_scores[-5000:]
    close_scores[:] = close_scores[-5000:]
    genetic_scores[:] = genetic_scores[-5000:]


    smart_distance.extend(difference_distance(latest_smart_start_x, latest_smart_start_y, latest_high_start_x, latest_high_start_y, latest_close_start_x, latest_close_start_y, latest_genetic_start_x, latest_genetic_start_y))
    high_distance.extend(difference_distance(latest_high_start_x, latest_high_start_y, latest_smart_start_x, latest_smart_start_y, latest_close_start_x, latest_close_start_y, latest_genetic_start_x, latest_genetic_start_y))
    close_distance.extend(difference_distance(latest_close_start_x, latest_close_start_y, latest_smart_start_x, latest_smart_start_y, latest_high_start_x, latest_high_start_y, latest_genetic_start_x, latest_genetic_start_y))
    genetic_distance.extend(difference_distance(latest_genetic_start_x, latest_genetic_start_y, latest_smart_start_x, latest_smart_start_y, latest_high_start_x, latest_high_start_y, latest_close_start_x, latest_close_start_y))

    smart_distance[:] = smart_distance[-5000:]  
    high_distance[:] = high_distance[-5000:]
    close_distance[:] = close_distance[-5000:]
    genetic_distance[:] = genetic_distance[-5000:]


    x = range(len(smart_scores))

    smart_line.set_data(x, smart_scores)
    high_line.set_data(x, high_scores)
    close_line.set_data(x, close_scores)
    genetic_line.set_data(x, genetic_scores)

    smart_eucl.set_data(x, smart_distance)
    high_eucl.set_data(x, high_distance)
    close_eucl.set_data(x, close_distance)
    genetic_eucl.set_data(x, close_distance)

    ax1.set_xlim(max(0, len(smart_scores) - 5000), len(smart_scores))
    ax2.set_xlim(max(0, len(smart_scores) - 5000), len(smart_scores))

    return smart_line, high_line, close_line, genetic_line, smart_eucl, high_eucl, close_eucl, genetic_eucl

def difference_distance(x1, y1, x2, y2, x3, y3, x4, y4):
    return np.sqrt((x2 - x1)**2 +(y2 - y1)**2) + np.sqrt(((x3 - x1)**2 +(y3 - y1)**2)) + np.sqrt(((x4 - x1)**2 +(y4 - y1)**2)) 

ani = FuncAnimation(fig, update, init_func=init, interval=100, blit=True)

plt.tight_layout()
plt.show()